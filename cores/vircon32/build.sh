#!/bin/sh
set -eu

commit="$2"
if [ "$1" = stamp ]; then
    echo "commit=$commit"
    echo "source=https://github.com/vircon32/vircon32-libretro/archive/$commit.tar.gz"
    echo "recipe=cmake-3.28 release gles2 cortex-a53 glibc-2.31"
    exit
fi

[ "$1" = build ] && [ $# -eq 5 ] || exit 2
tarball="$3"
work="$4"
out="$5"
src="$work/vircon32-libretro-$commit"
build="$work/build"

rm -rf "$work"
mkdir -p "$work"
tar -xzf "$tarball" -C "$work"
cmake -S "$src" -B "$build" \
    -DCMAKE_BUILD_TYPE=Release \
    -DENABLE_OPENGLES2=1 \
    -DOPENGL_LIBRARIES=GLESv2 \
    -DCMAKE_C_FLAGS_RELEASE="-O3 -DNDEBUG -mcpu=cortex-a53" \
    -DCMAKE_CXX_FLAGS_RELEASE="-O3 -DNDEBUG -mcpu=cortex-a53"
cmake --build "$build" --parallel "$(getconf _NPROCESSORS_ONLN)"
mkdir -p "$(dirname "$out")"
cp "$build/vircon32_libretro.so" "$out"
sh "$0" stamp "$commit" >"$out.meta"
