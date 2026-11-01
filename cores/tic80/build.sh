#!/bin/sh
set -eu

commit="$2"
if [ "$1" = stamp ]; then
    echo "commit=$commit"
    echo "engine_commit=${TIC80_ENGINE_COMMIT:?}"
    echo "source=https://github.com/libretro/TIC-80/tree/$commit"
    echo "recipe=cmake BUILD_LIBRETRO=ON; cmake --build"
    exit
fi

[ "$1" = build ] && [ $# -eq 5 ] || exit 2
tarball="$3"
work="$4"
out="$5"
src="$work/TIC-80-libretro-$commit/core"

rm -rf "$work"
mkdir -p "$work"
tar -xzf "$tarball" -C "$work"
cmake -S "$src" -B "$work/build" \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_C_FLAGS_RELEASE='-O3 -DNDEBUG -mcpu=cortex-a53' \
    -DCMAKE_CXX_FLAGS_RELEASE='-O3 -DNDEBUG -mcpu=cortex-a53' \
    -DBUILD_PLAYER=OFF -DBUILD_SDL=OFF -DBUILD_TOOLS=OFF -DBUILD_EDITORS=OFF \
    -DBUILD_LIBRETRO=ON -DBUILD_WITH_ALL=ON \
    -DBUILD_WITH_RUBY=OFF -DBUILD_WITH_PYTHON=OFF -DBUILD_WITH_JANET=OFF \
    -DBUILD_WITH_SCHEME=OFF
cmake --build "$work/build" --target tic80_libretro -j"$(getconf _NPROCESSORS_ONLN)"
mkdir -p "$(dirname "$out")"
cp "$work/build/bin/tic80_libretro.so" "$out"
sh "$0" stamp "$commit" >"$out.meta"
