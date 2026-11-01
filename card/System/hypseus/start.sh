#!/bin/sh
set -eu

SD="${OMEGA_ROOT:-/mnt/sdcard}"
HYPSEUS="$SD/System/hypseus"
REQUEST="${1:-$SD/System/hypseus.launch}"

IFS= read -r GAME < "$REQUEST"
case "$GAME" in
	"$SD/Games/DAPHNE/"*.daphne) ;;
	*) echo "Hypseus: invalid game path" >&2; exit 2 ;;
esac
[ -d "$GAME" ] || { echo "Hypseus: game directory not found: $GAME" >&2; exit 2; }
[ -x "$HYPSEUS/hypseus" ] || { echo "Hypseus is not installed" >&2; exit 127; }

DRIVER="$(basename "$GAME" .daphne)"
case "$DRIVER" in
	*[!A-Za-z0-9_]*) echo "Hypseus: invalid driver: $DRIVER" >&2; exit 2 ;;
esac
FRAMEFILE="$GAME/$DRIVER.txt"
[ -f "$FRAMEFILE" ] || { echo "Hypseus: framefile not found: $FRAMEFILE" >&2; exit 2; }

export HOME="$HYPSEUS/home"
export LD_LIBRARY_PATH="$HYPSEUS/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
export SDL_VIDEODRIVER=mali
export SDL_AUDIODRIVER=alsa
export SDL_VIDEO_EGL_DRIVER=/usr/lib/libEGL.so.1
mkdir -p "$HOME"

exec "$HYPSEUS/hypseus" "$DRIVER" vldp \
	-framefile "$FRAMEFILE" \
	-romdir "$GAME" \
	-homedir "$HYPSEUS" \
	-datadir "$HYPSEUS" \
	-fullscreen -x 640 -y 480 -force_aspect_ratio -gamepad
