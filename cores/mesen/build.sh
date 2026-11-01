#!/bin/sh
set -eu

device_cflags="-mcpu=cortex-a53"
device_ldflags="-static-libstdc++ -static-libgcc"

usage() {
	echo "usage: $0 stamp COMMIT | build COMMIT TARBALL WORKDIR OUT" >&2
	exit 2
}

stamp() {
	echo "commit=$1"
	echo "source=https://github.com/libretro/Mesen/archive/$1.tar.gz"
	echo "recipe=make -C Libretro platform=unix"
	echo "patch=callbacks-after-init-v2"
	echo "device_cflags=$device_cflags"
	echo "device_ldflags=$device_ldflags"
}

build() {
	commit="$1" tarball="$2" work="$3" out="$4"
	src="$work/Mesen-$commit"

	rm -rf "$work"
	mkdir -p "$work"
	tar -xzf "$tarball" -C "$work"
	if [ ! -f "$src/Libretro/Makefile" ]; then
		echo "$tarball does not hold Mesen-$commit/Libretro/Makefile" >&2
		exit 1
	fi
	patch -d "$src" -p1 < cores/mesen/callbacks-after-init.patch

	CFLAGS="$device_cflags" CXXFLAGS="$device_cflags" LDFLAGS="$device_ldflags" \
		make -C "$src/Libretro" platform=unix -j"$(getconf _NPROCESSORS_ONLN)"

	mkdir -p "$(dirname "$out")"
	cp "$src/Libretro/mesen_libretro.so" "$out"
	stamp "$commit" >"$out.meta"
}

case "${1:-}" in
stamp)
	[ $# -eq 2 ] && [ -n "$2" ] || usage
	stamp "$2"
	;;
build)
	[ $# -eq 5 ] && [ -n "$2" ] && [ -n "$3" ] && [ -n "$4" ] && [ -n "$5" ] || usage
	build "$2" "$3" "$4" "$5"
	;;
*)
	usage
	;;
esac
