#!/bin/sh
# Ağ servisinin, eylemlerinin ve Wi-Fi/VPN sayfa içeriğinin uçtan uca testi
# (docs/design/network-pages.md "Test").
#
#   sh tools/network-dbus-test.sh <network_dbus test ikilisi> <fake_nm ikilisi> <boş çalışma dizini>
#
# İkilileri önce derleyin (cargo-gate ile):
#   cargo test --features test-support --test network_dbus --no-run
#   cargo build --locked --manifest-path ../shell/crates/nuxut-nm/Cargo.toml --features test-support \
#       --example fake_nm --target-dir target/fake-nm      # crate'in sahte NM'si (kopya yok)
# Betik ortamı yalıtır (Wayland/Hyprland değişkenleri silinir, geçici XDG_CONFIG_HOME +
# DCONF_PROFILE, GDK_DEBUG=no-portals), SONRA kendini `dbus-run-session` içinde yeniden
# çalıştırır; her senaryo ayrı süreç. Gerçek sistem NM'ine hiçbir çağrı gitmez
# (NUXUT_NM_BUS=session; sahte NM yalnız NUXUT_FAKE_BUS=1 ve yalıtılmış bus'ta açılır). Sahte
# NM'nin çağrı logundan denetler: Ayarlar ajan kaydetmez; tarama yalnız sayfa görünürken ve bir
# kez; her eylemin NM çağrısı (şifreler test sabiti).
set -eu

if [ "${1:-}" != --inner ]; then
    [ $# -eq 3 ] || { echo "kullanım: $0 <test ikilisi> <fake_nm> <çalışma dizini>" >&2; exit 2; }
    mkdir -p "$3"
    work=$(cd "$3" && pwd)
    unset WAYLAND_DISPLAY HYPRLAND_INSTANCE_SIGNATURE DISPLAY
    export XDG_CONFIG_HOME="$work/config" DCONF_PROFILE="$work/dconf-none" GDK_DEBUG=no-portals
    exec dbus-run-session -- sh "$0" --inner "$1" "$2" "$work"
fi

test_bin=$2
fake_bin=$3
work=$4
case "${DBUS_SESSION_BUS_ADDRESS:-}" in
    "" | *"${XDG_RUNTIME_DIR:-/run/user/x}/bus"* | unix:abstract=*) echo "yalıtılmış oturum bus'ı yok; durduruldu" >&2; exit 1 ;;
esac
export NUXUT_NM_BUS=session

fake_pid=""
# Senaryo sonunda sahte NM'yi durdurur. `set -e`'yi KAPATMAZ: eskiden senaryo arasında çağrılan
# `cleanup` `set +e` bırakıyordu ve ilk senaryodan sonra test hataları betiği durdurmuyordu.
stop_fake() {
    if [ -n "$fake_pid" ]; then
        kill "$fake_pid" 2>/dev/null || true
        wait "$fake_pid" 2>/dev/null || true
    fi
    fake_pid=""
}
cleanup() {
    set +e
    stop_fake
}
trap cleanup EXIT INT TERM

# $1 senaryo (test adı), $2 sahne (none = sahte NM yok, nobus = ulaşılamayan bus, noaddr = adres
# yok; demo = sahte NM `default` sahnesi; no-device, empty, wifi-off = sahte NM'nin o sahnesi —
# NUXUT_FAKE_NM_SCENE), $3 beklenen RequestScan sayısı.
scenario() {
    name=$1
    scene=$2
    scans=$3
    dir="$work/$name"
    mkdir -p "$dir"
    : > "$dir/fake.log"
    case "$scene" in none | nobus | noaddr) fake_scene="" ;; demo) fake_scene=default ;; *) fake_scene=$scene ;; esac
    if [ -n "$fake_scene" ]; then
        NUXUT_FAKE_BUS=1 NUXUT_FAKE_NM_SCENE="$fake_scene" "$fake_bin" > "$dir/fake.log" 2>&1 &
        fake_pid=$!
        i=0
        until grep -q "hazır" "$dir/fake.log" 2>/dev/null; do
            i=$((i + 1))
            [ $i -lt 200 ] || { echo "$name: sahte NM başlamadı" >&2; cat "$dir/fake.log" >&2; exit 1; }
            sleep 0.05
        done
    fi
    echo "== $name"
    case "$scene" in
        nobus) HOME="$dir" DBUS_SESSION_BUS_ADDRESS="unix:path=$dir/yok.sock" "$test_bin" --ignored --exact "$name" --test-threads=1 ;;
        noaddr) (unset DBUS_SESSION_BUS_ADDRESS; HOME="$dir" exec "$test_bin" --ignored --exact "$name" --test-threads=1) ;;
        *) HOME="$dir" "$test_bin" --ignored --exact "$name" --test-threads=1 ;;
    esac
    stop_fake
    if grep -q "Register" "$dir/fake.log"; then
        echo "$name: Ayarlar gizli bilgi ajanı kaydetti (yasak)" >&2
        exit 1
    fi
    got=$(grep -c "RequestScan" "$dir/fake.log" || true)
    [ "$got" -eq "$scans" ] || { echo "$name: $got tarama, beklenen $scans" >&2; exit 1; }
    return 0
}

# $1 senaryo, $2 sabit metin: sahte NM logunda olmalı.
expect() {
    grep -qF -- "$2" "$work/$1/fake.log" || { echo "$1: logda yok: $2" >&2; cat "$work/$1/fake.log" >&2; exit 1; }
}

# $1 senaryo, $2 sabit metin: sahte NM logunda OLMAMALI.
reject() {
    if grep -qF -- "$2" "$work/$1/fake.log"; then
        echo "$1: logda olmamalı: $2" >&2
        exit 1
    fi
}

scenario missing_service none 0
scenario bus_unreachable nobus 0
scenario no_session_address noaddr 0
scenario connected demo 1
scenario scan_when_opened_on_wifi demo 1

scenario wifi_toggle demo 0
expect wifi_toggle "WirelessEnabled false"
expect wifi_toggle "WirelessEnabled true"

scenario connect_open demo 0
expect connect_open "AddAndActivate Misafir - - /org/freedesktop/NetworkManager/Devices/1 /org/freedesktop/NetworkManager/AccessPoint/3"

scenario connect_password demo 0
expect connect_password "AddAndActivate Kafe wpa-psk test-sifre-1"

scenario connect_password_invalid demo 0
reject connect_password_invalid "AddAndActivate"

scenario saved_needs_password demo 0
expect saved_needs_password "Activate /org/freedesktop/NetworkManager/Settings/1"
expect saved_needs_password "WrongPassword /org/freedesktop/NetworkManager/Settings/1"
expect saved_needs_password "Update /org/freedesktop/NetworkManager/Settings/1 secrets=1"
reject saved_needs_password "Delete"
reject saved_needs_password "AddAndActivate Ev"
upd=$(grep -c "Update /org/freedesktop/NetworkManager/Settings/1 secrets=1" "$work/saved_needs_password/fake.log" || true)
[ "$upd" -eq 2 ] || { echo "saved_needs_password: $upd şifre güncellemesi, beklenen 2" >&2; exit 1; }
act=$(grep -c "^fake_nm: Activate /org/freedesktop/NetworkManager/Settings/1 " "$work/saved_needs_password/fake.log" || true)
[ "$act" -eq 3 ] || { echo "saved_needs_password: $act etkinleştirme, beklenen 3" >&2; exit 1; }

scenario hidden_network demo 0
expect hidden_network "hidden Gizli"
expect hidden_network "AddAndActivate Gizli wpa-psk gizli-sifre"

scenario disconnect demo 0
expect disconnect "Deactivate /org/freedesktop/NetworkManager/ActiveConnection/1"

scenario saved_settings_and_forget demo 0
expect saved_settings_and_forget "Update /org/freedesktop/NetworkManager/Settings/2 secrets=0"
expect saved_settings_and_forget "Delete /org/freedesktop/NetworkManager/Settings/2"

scenario vpn_toggle demo 0
expect vpn_toggle "Activate /org/freedesktop/NetworkManager/Settings/3"
reject vpn_toggle "Deactivate"
reject vpn_toggle "Delete"

scenario vpn_delete demo 0
expect vpn_delete "Delete /org/freedesktop/NetworkManager/Settings/3"

scenario wireguard_import demo 0
expect wireguard_import "AddConnection wireguard wg-test"
reject wireguard_import "AddConnection wireguard wg-bozuk"
reject wireguard_import "AddConnection wireguard bu-ad"

scenario vpn_off demo 0
expect vpn_off "Activate /org/freedesktop/NetworkManager/Settings/3"
expect vpn_off "Deactivate /org/freedesktop/NetworkManager/ActiveConnection/"

scenario scene_no_device no-device 0
scenario scene_empty empty 1
scenario scene_wifi_off wifi-off 1
expect scene_wifi_off "WirelessEnabled true"

echo "network-dbus: tüm senaryolar geçti"
