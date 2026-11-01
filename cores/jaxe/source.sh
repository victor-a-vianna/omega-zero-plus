#!/bin/sh
set -eu

[ $# -eq 3 ] || exit 2
commit="$1"
work="$2"
out="$3"
src="$work/jaxe-$commit"

rm -rf "$work"
mkdir -p "$work"
git clone --filter=blob:none --no-checkout https://github.com/kurtjd/jaxe.git "$src"
git -C "$src" fetch --depth 1 origin "$commit"
git -C "$src" checkout --detach "$commit"
git -C "$src" submodule update --init --recursive --depth 1
COPYFILE_DISABLE=1 tar --no-xattrs --format pax --exclude='.git' --exclude='*/.git' \
    -czf "$out" -C "$work" "jaxe-$commit"
