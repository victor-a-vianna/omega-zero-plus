#!/bin/sh
set -eu

commit="$2"
if [ "$1" = stamp ]; then
    echo "commit=$commit"
    echo "source=https://github.com/libretro/NP2kai/archive/$commit.tar.gz"
    echo "recipe=make -C sdl -f Makefile.libretro platform=unix"
    exit
fi

[ "$1" = build ] && [ $# -eq 5 ] || exit 2
tarball="$3"
work="$4"
out="$5"
src="$work/NP2kai-$commit"

rm -rf "$work"
mkdir -p "$work"
tar -xzf "$tarball" -C "$work"
CFLAGS="-mcpu=cortex-a53" CXXFLAGS="-mcpu=cortex-a53" \
    make -C "$src/sdl" -f Makefile.libretro platform=unix \
    NP2KAI_HASH="$(printf %s "$commit" | cut -c1-7)" \
    -j"$(getconf _NPROCESSORS_ONLN)"
mkdir -p "$(dirname "$out")"
cp "$src/sdl/np2kai_libretro.so" "$out"
sh "$0" stamp "$commit" >"$out.meta"
