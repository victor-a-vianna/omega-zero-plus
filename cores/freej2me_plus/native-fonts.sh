#!/bin/sh
set -eu

[ $# -eq 1 ] || exit 2
out="$1"

rm -rf "$out"
mkdir -p "$out"
for name in \
    libfontconfig.so.1 \
    libfreetype.so.6 \
    libexpat.so.1 \
    libz.so.1 \
    libbz2.so.1.0 \
    libpng16.so.16 \
    libbrotlidec.so.1 \
    libbrotlicommon.so.1
do
    path="$(ldconfig -p | awk -v lib="$name" '$1 == lib { print $NF; exit }')"
    [ -n "$path" ]
    cp -L "$path" "$out/$name"
done
