#!/bin/sh
set -eu

SD="${OMEGA_ROOT:-/mnt/sdcard}"
BIGPEMU="$SD/System/bigpemu"
REQUEST="${1:-$SD/System/bigpemu.launch}"

IFS= read -r ROM < "$REQUEST"
case "$ROM" in
	"$SD/Games/JAGUAR/"*|"$SD/Games/JAGUARCD/"*) ;;
	*) echo "BigPEmu: invalid ROM path" >&2; exit 2 ;;
esac
[ -f "$ROM" ] || { echo "BigPEmu: ROM not found: $ROM" >&2; exit 2; }
[ -x "$BIGPEMU/bigpemu" ] || { echo "BigPEmu is not installed" >&2; exit 127; }

export LD_LIBRARY_PATH="$BIGPEMU${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
export LIBGL_ES=2
export SDL_VIDEODRIVER=mali
export SDL_AUDIODRIVER=alsa
export SDL_VIDEO_EGL_DRIVER=/usr/lib/libEGL.so.1
export HOME="$BIGPEMU/bigpemu_userdata"
mkdir -p "$HOME"
if [ -f "$BIGPEMU/libGL.so.1" ]; then
	export LD_PRELOAD="$BIGPEMU/libGL.so.1${LD_PRELOAD:+:$LD_PRELOAD}"
fi
cd "$BIGPEMU"
INPUT_PID=""
cleanup_input() {
	[ -z "$INPUT_PID" ] || kill "$INPUT_PID" 2>/dev/null || true
	[ -z "$INPUT_PID" ] || wait "$INPUT_PID" 2>/dev/null || true
}
trap cleanup_input EXIT INT TERM
if [ -x "$BIGPEMU/evdev-keyboard" ]; then
	"$BIGPEMU/evdev-keyboard" &
	INPUT_PID=$!
	sleep 0.2
fi
if [ -x "$BIGPEMU/sdl-input-probe" ]; then
	"$BIGPEMU/sdl-input-probe" || true
fi
set +e
"$BIGPEMU/bigpemu" "$ROM" -localdata -forcewidth 640 -forceheight 480
STATUS=$?
set -e
exit "$STATUS"
