#!/bin/sh
set -eu

device_cflags="-mcpu=cortex-a53"

usage() {
	echo "usage: $0 stamp COMMIT | build COMMIT TARBALL WORKDIR OUT" >&2
	exit 2
}

stamp() {
	echo "commit=$1"
	echo "source=https://github.com/notaz/picodrive/archive/$1.tar.gz"
	echo "recipe=make -f Makefile.libretro platform=unix"
	echo "device_cflags=$device_cflags"
}

build() {
	commit="$1" tarball="$2" work="$3" out="$4"
	src="$work/picodrive-$commit"
	rm -rf "$work"
	mkdir -p "$work"
	tar -xzf "$tarball" -C "$work"
	[ -f "$src/Makefile.libretro" ] || {
		echo "$tarball does not hold picodrive-$commit/Makefile.libretro" >&2
		exit 1
	}
	# PicoDrive forwards GIT_VERSION through recursive make invocations. A value
	# containing a leading space is split there and becomes a bogus make target.
	CFLAGS="$device_cflags" make -C "$src" -f Makefile.libretro platform=unix \
		GIT_VERSION="$(printf %s "$commit" | cut -c1-7)" -j"$(getconf _NPROCESSORS_ONLN)"
	mkdir -p "$(dirname "$out")"
	cp "$src/picodrive_libretro.so" "$out"
	stamp "$commit" >"$out.meta"
}

case "${1:-}" in
stamp) [ $# -eq 2 ] && stamp "$2" || usage ;;
build) [ $# -eq 5 ] && build "$2" "$3" "$4" "$5" || usage ;;
*) usage ;;
esac
