#!/bin/sh
set -eu

[ $# -eq 4 ] || exit 2
commit="$1"
work="$2"
out="$3"
engine_commit="$4"
src="$work/TIC-80-libretro-$commit"

rm -rf "$work"
mkdir -p "$work"
git clone --filter=blob:none --no-checkout https://github.com/libretro/TIC-80.git "$src"
git -C "$src" fetch --depth 1 origin "$commit"
git -C "$src" checkout --detach "$commit"
git -C "$src" submodule update --init --recursive --depth 1
git -C "$src/core" fetch --depth 1 origin "$engine_commit"
git -C "$src/core" checkout --detach "$engine_commit"
git -C "$src/core" submodule update --init --recursive --depth 1
# Some upstream example files are checked out without owner-read permission; they are source
# payload, not secrets, and GNU tar in the ARM builder must be able to archive them.
find "$src/core" -type d -exec chmod u+rwx {} \;
find "$src/core" -type f -exec chmod u+rw {} \;
# macOS otherwise stores com.apple.* metadata as PAX records. GNU tar in the
# Linux device build treats some of those records as extraction errors.
COPYFILE_DISABLE=1 tar -h --no-xattrs --format pax --exclude='.git' --exclude='*/.git' \
    --exclude='*/core/vendor/sdl2' --exclude='*/core/vendor/wasm3/platforms' \
    -czf "$out" -C "$work" "TIC-80-libretro-$commit"
