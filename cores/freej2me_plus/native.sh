#!/bin/sh
set -eu

[ $# -eq 4 ] || exit 2
commit="$1"
tarball="$2"
work="$3"
out="$4"
src="$work/freej2me-plus-$commit"

rm -rf "$work"
mkdir -p "$work"
tar -xzf "$tarball" -C "$work"
patch -d "$src" -p1 < cores/freej2me_plus/fontconfig.patch
patch -d "$src" -p1 < cores/freej2me_plus/nonblocking.patch

make -C "$src/src/libretro" clean
make -C "$src/src/libretro" -j"$(getconf _NPROCESSORS_ONLN)" \
    CFLAGS='-O3 -DNDEBUG -mcpu=cortex-a53 -Wall -Wno-unused-result -D__LIBRETRO__ -Ideps/libretro-common/include -fPIC' \
    CXXFLAGS='-O3 -DNDEBUG -mcpu=cortex-a53 -Wall -Wno-unused-result -D__LIBRETRO__ -Ideps/libretro-common/include -fPIC'

mkdir -p "$(dirname "$out")"
cp "$src/src/libretro/freej2me_plus_libretro.so" "$out"
