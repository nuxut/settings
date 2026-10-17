#!/bin/sh
# Kullanıcı hesabı servisinin uçtan uca testi (docs/design/account.md "Test").
#
#   sh tools/accounts-dbus-test.sh <accounts_dbus test ikilisi> <fake_accounts ikilisi> <boş çalışma dizini>
#
# İkilileri önce derleyin (cargo-gate ile): `cargo test --test accounts_dbus --no-run` ve
# `cargo build --example fake_accounts`. Betik ortamı yalıtır (Wayland/Hyprland değişkenleri
# silinir, geçici XDG_CONFIG_HOME + DCONF_PROFILE, GDK_DEBUG=no-portals), SONRA kendini
# `dbus-run-session` içinde yeniden çalıştırır; her senaryo ayrı süreç, ayrı geçici HOME.
# Gerçek sistem bus'ına / AccountsService'e hiçbir çağrı gitmez (NUXUT_ACCOUNTS_BUS=session).
set -eu

if [ "${1:-}" != --inner ]; then
    [ $# -eq 3 ] || { echo "kullanım: $0 <test ikilisi> <fake_accounts> <çalışma dizini>" >&2; exit 2; }
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
    "" | *"${XDG_RUNTIME_DIR:-/run/user/x}/bus"*) echo "yalıtılmış oturum bus'ı yok; durduruldu" >&2; exit 1 ;;
esac
export NUXUT_ACCOUNTS_BUS=session

fake_pid=""
cleanup() {
    set +e
    if [ -n "$fake_pid" ]; then
        kill "$fake_pid" 2>/dev/null
        wait "$fake_pid" 2>/dev/null
    fi
    fake_pid=""
}
trap cleanup EXIT INT TERM

# $1 senaryo adı (test adı), $2 sahte servis: on|off; kalan: sahte servisin ortamı (A=B).
scenario() {
    name=$1
    fake=$2
    shift 2
    dir="$work/$name"
    mkdir -p "$dir/home" "$dir/icons"
    if [ "$fake" = on ]; then
        env NUXUT_FAKE_ACCOUNTS_DIR="$dir/icons" "$@" "$fake_bin" > "$dir/fake.log" 2>&1 &
        fake_pid=$!
        i=0
        until grep -q "hazır" "$dir/fake.log" 2>/dev/null; do
            i=$((i + 1))
            [ $i -lt 200 ] || { echo "$name: sahte servis başlamadı" >&2; cat "$dir/fake.log" >&2; exit 1; }
            sleep 0.05
        done
    fi
    echo "== $name"
    HOME="$dir/home" XDG_CACHE_HOME="$dir/home/.cache" "$test_bin" --ignored --exact "$name" --test-threads=1
    cleanup
    if [ "$fake" = on ]; then
        grep -q "interactive=false" "$dir/fake.log" && { echo "$name: yetki bayrağı olmadan yazma çağrısı" >&2; exit 1; }
    fi
    return 0
}

scenario missing_service off
scenario read_and_write on NUXUT_FAKE_ACCOUNTS_ADMIN=1
scenario denied_is_silent on NUXUT_FAKE_ACCOUNTS_DENY=1 NUXUT_FAKE_ACCOUNTS_DELAY_MS=300
echo "accounts-dbus: tüm senaryolar geçti"
