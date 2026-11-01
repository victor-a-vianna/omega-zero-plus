#!/bin/sh
set -eu

commit="$2"
if [ "$1" = stamp ]; then
    echo "commit=$commit"
    echo "source=https://github.com/libretro/arduous.git (recursive)"
    echo "recipe=cmake release; cmake --build"
    exit
fi

[ "$1" = build ] && [ $# -eq 5 ] || exit 2
tarball="$3"
work="$4"
out="$5"
src="$work/arduous-$commit"

rm -rf "$work"
mkdir -p "$work"
tar -xzf "$tarball" -C "$work"
cmake -S "$src" -B "$src/build-device" \
    -DCMAKE_BUILD_TYPE=Release \
    -DWERROR=OFF \
    -DLIBRETRO=ON \
    -DCMAKE_C_FLAGS=-mcpu=cortex-a53 \
    -DCMAKE_CXX_FLAGS=-mcpu=cortex-a53
cmake --build "$src/build-device" --parallel "$(getconf _NPROCESSORS_ONLN)"
mkdir -p "$(dirname "$out")"
cp "$src/build-device/arduous_libretro.so" "$out"
sh "$0" stamp "$commit" >"$out.meta"
