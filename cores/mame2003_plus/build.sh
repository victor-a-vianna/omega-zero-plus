#!/bin/sh
set -eu

commit="$2"
if [ "$1" = stamp ]; then
    echo "commit=$commit"
    echo "source=https://github.com/libretro/mame2003-plus-libretro/archive/$commit.tar.gz"
    echo "recipe=make platform=unix"
    exit
fi

[ "$1" = build ] && [ $# -eq 5 ] || exit 2
tarball="$3"
work="$4"
out="$5"
src="$work/mame2003-plus-libretro-$commit"

rm -rf "$work"
mkdir -p "$work"
tar -xzf "$tarball" -C "$work"
CFLAGS="-O3 -DNDEBUG -mcpu=cortex-a53" \
    CXXFLAGS="-O3 -DNDEBUG -mcpu=cortex-a53" \
    make -C "$src" platform=unix -j"$(getconf _NPROCESSORS_ONLN)"
mkdir -p "$(dirname "$out")"
cp "$src/mame2003_plus_libretro.so" "$out"
sh "$0" stamp "$commit" >"$out.meta"
