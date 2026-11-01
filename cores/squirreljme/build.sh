#!/bin/sh
set -eu

commit="$2"
if [ "$1" = stamp ]; then
    echo "commit=$commit"
    echo "source=https://github.com/SquirrelJME/SquirrelJME/archive/$commit.tar.gz"
    echo "recipe=cmake nanocoat SQUIRRELJME_SPECIAL_BUILD_LIBRETRO=1"
    exit
fi

[ "$1" = build ] && [ $# -eq 6 ] || exit 2
tarball="$3"
work="$4"
out="$5"
classlib_out="$6"
src="$work/SquirrelJME-$commit"

rm -rf "$work"
mkdir -p "$work"
tar -xzf "$tarball" -C "$work"
src="$(cd "$src" && pwd)"
SQUIRRELJME_SPECIAL_BUILD_LIBRETRO=1 cmake \
    -S "$src/nanocoat" -B "$work/build" \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_C_FLAGS_RELEASE="-O3 -DNDEBUG -mcpu=cortex-a53"
cmake --build "$work/build" --target squirreljme_libretro \
    -j"$(getconf _NPROCESSORS_ONLN)"
mkdir -p "$(dirname "$out")"
cp "$work/build/bin/squirreljme_libretro.so" "$out"
JAVA_TOOL_OPTIONS="-Dsquirreljme.emulator.libpath=$src/emulators/emulator-base/build/cmake-libNatives/bin/libemulator-base.so" \
    "$src/gradlew" -p "$src" --no-daemon romSpringCoatRelease
mkdir -p "$(dirname "$classlib_out")"
cp "$src/build/squirreljme/vm-main-springcoat-release/squirreljme.jar" \
    "$classlib_out"
sh "$0" stamp "$commit" >"$out.meta"
