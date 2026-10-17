#!/bin/sh
# Bluetooth servisinin ve sayfa içeriğinin uçtan uca testi (docs/design/bluetooth-page.md "Test").
#
#   sh tools/bluetooth-dbus-test.sh <bluetooth_dbus test ikilisi> <boş, KISA çalışma dizini>
#
# İkiliyi önce derleyin (cargo-gate ile):
#   cargo test --features test-support --test bluetooth_dbus --no-run
# Sahte BlueZ (nuxut-bluez'in paylaşılan tests/fake_bluez'i) test sürecinin İÇİNDE çalışır.
# Betik ortamı yalıtır (Wayland/Hyprland değişkenleri silinir, geçici XDG_CONFIG_HOME +
# DCONF_PROFILE, GDK_DEBUG=no-portals), SONRA kendini `dbus-run-session` içinde yeniden
# çalıştırır; her senaryo ayrı süreç (servis durumu süreç geneli). Gerçek sistem BlueZ'ine hiçbir
# çağrı gitmez (NUXUT_BLUEZ_BUS=session; rfkill sysfs yerine geçici dizin). Ajan kaydı olmadığı
# her senaryoda test içinde denetlenir. Eşleştirme öncesi kabuk sorgusu (`msg bluetooth
# agent-status`) yalnız test örneğine (NUXUT_SHELL_INSTANCE=stbttest) ve senaryonun geçici
# XDG_RUNTIME_DIR'ine gider; sahte kabuk test sürecinin içindedir (pairing_agent_warning).
set -eu

if [ "${1:-}" != --inner ]; then
    [ $# -eq 2 ] || { echo "kullanım: $0 <test ikilisi> <çalışma dizini>" >&2; exit 2; }
    mkdir -p "$2"
    work=$(cd "$2" && pwd)
    unset WAYLAND_DISPLAY HYPRLAND_INSTANCE_SIGNATURE DISPLAY
    export XDG_CONFIG_HOME="$work/config" DCONF_PROFILE="$work/dconf-none" GDK_DEBUG=no-portals
    exec dbus-run-session -- sh "$0" --inner "$1" "$work"
fi

test_bin=$2
work=$3
case "${DBUS_SESSION_BUS_ADDRESS:-}" in
    "" | *"${XDG_RUNTIME_DIR:-/run/user/x}/bus"* | *"/run/user/"*) echo "yalıtılmış oturum bus'ı yok; durduruldu" >&2; exit 1 ;;
esac
export NUXUT_BLUEZ_BUS=session

scenario() {
    name=$1
    dir="$work/$name"
    mkdir -p "$dir/rfkill" "$dir/run"
    # Unix soket yolu ≤ 107 bayt: çalışma dizini kısa olmalı (ör. /tmp/btw).
    sock="$dir/run/nuxut/nuxut-shell-stbttest.sock"
    [ ${#sock} -le 100 ] || { echo "$name: çalışma dizini çok uzun (soket yolu ${#sock} bayt); kısa dizin verin" >&2; exit 1; }
    chmod 700 "$dir/run"
    echo "== $name"
    # Kabuk IPC'si (eşleştirme öncesi ajan sorgusu) yalnız test örneğine ve geçici çalışma
    # dizinine: kullanıcının çalışan kabuğuna asla ulaşmaz.
    HOME="$dir" XDG_RUNTIME_DIR="$dir/run" NUXUT_SHELL_INSTANCE=stbttest NUXUT_BLUEZ_RFKILL_DIR="$dir/rfkill" \
        "$test_bin" --ignored --exact "$name" --test-threads=1
}

scenario missing_service
scenario no_adapter
scenario scene_off
scenario scene_empty
scenario scene_rfkill
scenario nearby_sorted_by_rssi
scenario power_and_rfkill
scenario name_and_visibility
scenario discovery_follows_page
scenario pairing_results
scenario pairing_agent_warning
scenario device_actions_and_battery
echo "bluetooth-dbus: tüm senaryolar geçti"
