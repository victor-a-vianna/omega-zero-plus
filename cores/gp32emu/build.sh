#!/bin/sh
set -eu

commit="$2"
if [ "$1" = stamp ]; then
    echo "commit=$commit"
    echo "source=https://github.com/gameblabla/gp32emu/archive/$commit.tar.gz"
    echo "recipe=make -f Makefile.libretro GP32EMU_ENABLE_THREADS=1"
    exit
fi

[ "$1" = build ] && [ $# -eq 5 ] || exit 2
tarball="$3"
work="$4"
out="$5"
src="$work/gp32emu-$commit"

rm -rf "$work"
mkdir -p "$work"
tar -xzf "$tarball" -C "$work"
CFLAGS="-std=c11 -O3 -DNDEBUG -fPIC -mcpu=cortex-a53" \
    make -C "$src" -f Makefile.libretro GP32EMU_ENABLE_THREADS=1 \
    -j"$(getconf _NPROCESSORS_ONLN)"
mkdir -p "$(dirname "$out")"
cp "$src/gp32emu_libretro.so" "$out"
sh "$0" stamp "$commit" >"$out.meta"
