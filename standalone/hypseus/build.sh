#!/bin/sh
set -eu

commit="$2"
if [ "$1" = stamp ]; then
	echo "commit=$commit"
	echo "source=https://github.com/DirtBagXon/hypseus-singe/archive/$commit.tar.gz"
	echo "recipe=cmake release sdl2 mali-fbdev-runtime"
	exit
fi

[ "$1" = build ] && [ $# -eq 6 ] || exit 2
tarball="$3"
work="$4"
out="$5"
sdl="$6"
src="$work/hypseus-singe-$commit"

rm -rf "$work" "$out"
mkdir -p "$work" "$out/lib"
tar -xzf "$tarball" -C "$work"
src="$(cd "$src" && pwd)"
build="$src/src/build-device"
sed -i "s|URL ../../../src/3rdparty/libmpeg2/libmpeg2-master.tgz|URL file://$src/src/3rdparty/libmpeg2/libmpeg2-master.tgz|" \
	"$src/src/cmake/modules/BuildLibMPEG2.cmake"
cmake -S "$src/src" -B "$build" \
	-DCMAKE_BUILD_TYPE=Release \
	-DCMAKE_C_FLAGS_RELEASE="-O3 -DNDEBUG -mcpu=cortex-a53" \
	-DCMAKE_CXX_FLAGS_RELEASE="-O3 -DNDEBUG -mcpu=cortex-a53"
cmake --build "$build" --parallel "$(getconf _NPROCESSORS_ONLN)"

cp "$build/hypseus" "$out/hypseus"
chmod 755 "$out/hypseus"
for data in fonts midi pics sound; do
	cp -R "$src/$data" "$out/$data"
done
cp "$src/LICENSE" "$out/LICENSE"
cp "$sdl" "$out/lib/libSDL2-2.0.so.0"

# Bundle only non-base shared libraries. The H700 image supplies glibc, libstdc++, ALSA and the
# Mali EGL/GLES stack; media/font/zip dependencies come from this reproducible Debian build.
lddtree -l "$out/hypseus" | while IFS= read -r library; do
	[ -f "$library" ] || continue
	name="$(basename "$library")"
	case "$name" in
		ld-linux-aarch64.so.1|libc.so.*|libm.so.*|libdl.so.*|libpthread.so.*|librt.so.*|libgcc_s.so.*|libstdc++.so.*|libasound.so.*|libSDL2-2.0.so.*) continue ;;
	esac
	cp -L "$library" "$out/lib/$name"
done
sh "$0" stamp "$commit" > "$out/hypseus.meta"
