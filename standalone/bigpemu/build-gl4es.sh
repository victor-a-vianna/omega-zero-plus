#!/bin/sh
set -eu

[ "$#" -eq 4 ] || {
	echo "usage: $0 COMMIT SOURCE.tar.gz BUILD_DIR OUTPUT" >&2
	exit 2
}
commit=$1
archive=$2
build=$3
output=$4

rm -rf "$build"
mkdir -p "$build/src" "$build/cmake" "$(dirname "$output")"
tar -xzf "$archive" -C "$build/src" --strip-components=1
cmake -S "$build/src" -B "$build/cmake" \
	-DODROID=1 -DNOX11=1 -DNOEGL=1 -DCMAKE_BUILD_TYPE=Release
cmake --build "$build/cmake" --parallel

library=$(find "$build" -type f -name 'libGL.so.1' -print | head -n 1)
[ -n "$library" ] || { echo "GL4ES did not produce libGL.so.1" >&2; exit 1; }
cp "$library" "$output"
printf 'commit=%s\noptions=ODROID,NOX11,NOEGL\n' "$commit" > "$output.meta"
