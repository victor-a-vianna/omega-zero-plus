#!/bin/sh
set -eu

commit="$2"
if [ "$1" = stamp ]; then
    echo "commit=$commit"
    echo "source=https://github.com/libretro/FBNeo/archive/$commit.tar.gz"
    echo "recipe=neogeocd-folder-state+stderr-log+make-rpi3_64-v2"
    exit
fi

[ "$1" = build ] && [ $# -eq 5 ] || exit 2
tarball="$3"
work="$4"
out="$5"
src="$work/FBNeo-$commit"

rm -rf "$work"
mkdir -p "$work"
tar -xzf "$tarball" -C "$work"
patch -d "$src" -p1 < cores/fbneo/0001-neogeocd-folder.patch
patch -d "$src" -p1 < cores/fbneo/0002-stderr-log.patch
make -C "$src/src/burner/libretro" platform=rpi3_64 -j"$(getconf _NPROCESSORS_ONLN)"
mkdir -p "$(dirname "$out")"
cp "$src/src/burner/libretro/fbneo_libretro.so" "$out"
sh "$0" stamp "$commit" >"$out.meta"
