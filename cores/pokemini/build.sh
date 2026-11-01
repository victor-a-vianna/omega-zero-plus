#!/bin/sh
set -eu

commit="$2"
if [ "$1" = stamp ]; then
    echo "commit=$commit"
    echo "source=https://github.com/libretro/PokeMini/archive/$commit.tar.gz"
    echo "recipe=make -f Makefile.libretro platform=unix"
    exit
fi

[ "$1" = build ] && [ $# -eq 5 ] || exit 2
tarball="$3"
work="$4"
out="$5"
src="$work/PokeMini-$commit"

rm -rf "$work"
mkdir -p "$work"
tar -xzf "$tarball" -C "$work"
CFLAGS="-mcpu=cortex-a53" CXXFLAGS="-mcpu=cortex-a53" \
    make -C "$src" -f Makefile.libretro platform=unix -j"$(getconf _NPROCESSORS_ONLN)"
mkdir -p "$(dirname "$out")"
cp "$src/pokemini_libretro.so" "$out"
sh "$0" stamp "$commit" >"$out.meta"
