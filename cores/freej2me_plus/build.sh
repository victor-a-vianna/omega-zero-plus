#!/bin/sh
set -eu

commit="$2"
if [ "$1" = stamp ]; then
    echo "commit=$commit"
    echo "source=https://github.com/TASEmulators/freej2me-plus/archive/$commit.tar.gz"
    echo "recipe=v10; self-contained font stack and nonblocking Java process"
    exit
fi

[ "$1" = build ] && [ $# -eq 6 ] || exit 2
tarball="$3"
work="$4"
core_out="$5"
jar_out="$6"
src="$work/freej2me-plus-$commit"

rm -rf "$work"
mkdir -p "$work"
tar -xzf "$tarball" -C "$work"

make -C "$src/src/libretro" clean
make -C "$src/src/libretro" -j"$(getconf _NPROCESSORS_ONLN)" \
    CFLAGS='-O3 -DNDEBUG -mcpu=cortex-a53 -Wall -Wno-unused-result -D__LIBRETRO__ -Ideps/libretro-common/include -fPIC' \
    CXXFLAGS='-O3 -DNDEBUG -mcpu=cortex-a53 -Wall -Wno-unused-result -D__LIBRETRO__ -Ideps/libretro-common/include -fPIC'
ant -f "$src/build.xml" jar-libretro

mkdir -p "$(dirname "$core_out")" "$(dirname "$jar_out")"
cp "$src/src/libretro/freej2me_plus_libretro.so" "$core_out"
cp "$src/build/freej2me_plus-lr.jar" "$jar_out"
sh "$0" stamp "$commit" >"$core_out.meta"
