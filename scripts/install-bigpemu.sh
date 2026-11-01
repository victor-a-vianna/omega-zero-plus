#!/bin/sh
set -eu

usage() {
	echo "usage: $0 BigPEmu_LinuxARM64_v1221.tar.gz ROOT [libOpenGL.so] [libSDL2-2.0.so.0]" >&2
	exit 2
}

[ "$#" -ge 2 ] && [ "$#" -le 4 ] || usage
archive=$1
root=$2
gl4es=${3:-}
sdl2=${4:-}
[ -f "$archive" ] || { echo "BigPEmu archive not found: $archive" >&2; exit 2; }
expected=a73cb33b27cf1178e487c9eb5b784bdcaac2209fe92a8ed85d3905d52a37a7e4
if command -v shasum > /dev/null 2>&1; then
	actual=$(shasum -a 256 "$archive" | awk '{print $1}')
else
	actual=$(sha256sum "$archive" | awk '{print $1}')
fi
[ "$actual" = "$expected" ] || {
	echo "invalid BigPEmu v1221 archive checksum: $actual" >&2
	exit 2
}

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
tar -tzf "$archive" > "$tmp/archive.list"
if grep -Eq '(^/|(^|/)\.\.(/|$))' "$tmp/archive.list"; then
	echo "invalid archive: unsafe path" >&2
	exit 2
fi
tar -xzf "$archive" -C "$tmp"

binary=$(find "$tmp" -type f -name bigpemu -print | head -n 1)
readme=$(find "$tmp" -type f \( -iname 'ReadMe.txt' -o -iname 'BigPEmu_ReadMe.txt' \) -print | head -n 1)
[ -n "$binary" ] || { echo "invalid archive: bigpemu is missing" >&2; exit 2; }
[ -n "$readme" ] || { echo "invalid archive: BigPEmu ReadMe/license is missing" >&2; exit 2; }
file "$binary" | grep -Eiq 'ELF 64-bit.*(ARM aarch64|aarch64)' || {
	echo "invalid archive: bigpemu is not Linux ARM64" >&2
	exit 2
}

source_dir=$(dirname "$binary")
destination="$root/System/bigpemu"
mkdir -p "$destination"
chmod -R u+w "$destination"
cp -R "$source_dir"/. "$destination"/
chmod 755 "$destination/bigpemu"
[ -f "$destination/$(basename "$readme")" ] || cp "$readme" "$destination/"
installer_dir=$(CDPATH= cd -- "$(dirname "$0")" && pwd)
cp "$installer_dir/../card/System/bigpemu/start.sh" "$destination/start.sh"
chmod 755 "$destination/start.sh"

if [ -n "$gl4es" ]; then
	[ -f "$gl4es" ] || { echo "GL4ES library not found: $gl4es" >&2; exit 2; }
	cp "$gl4es" "$destination/libGL.so.1"
	ln -sf libGL.so.1 "$destination/libOpenGL.so"
	ln -sf libOpenGL.so "$destination/libOpenGL.so.0"
fi

if [ -n "$sdl2" ]; then
	[ -f "$sdl2" ] || { echo "SDL2 library not found: $sdl2" >&2; exit 2; }
	file "$sdl2" | grep -Eiq 'ELF 64-bit.*(ARM aarch64|aarch64)' || {
		echo "invalid SDL2 library: not Linux ARM64" >&2
		exit 2
	}
	cp "$sdl2" "$destination/libSDL2-2.0.so.0"
fi

echo "BigPEmu installed in $destination"
[ -f "$destination/libOpenGL.so" ] || {
	echo "warning: GL4ES is not installed; the H700 build normally requires it" >&2
}
[ -f "$destination/libSDL2-2.0.so.0" ] || {
	echo "warning: SDL2 mali-fbdev is not installed; BigPEmu cannot start on BaseOS/H700" >&2
}
