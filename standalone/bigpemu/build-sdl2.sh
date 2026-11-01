#!/bin/sh
set -eu

[ "$#" -eq 5 ] || {
	echo "usage: $0 VERSION SOURCE.tar.gz PATCH_DIR BUILD_DIR OUTPUT" >&2
	exit 2
}
version=$1
archive=$2
patch_dir=$3
build=$4
output=$5

rm -rf "$build"
mkdir -p "$build/src" "$(dirname "$output")"
tar -xzf "$archive" -C "$build/src" --strip-components=1
for patch_file in "$patch_dir"/*.patch; do
	patch -d "$build/src" -p1 < "$patch_file"
done

# The H700 patches originated before SDL prefixed CMake options with SDL_.
sed -i 's/if(VIDEO_MALI)/if(SDL_MALI)/; s/endif(VIDEO_MALI)/endif(SDL_MALI)/; s/endif(HAVE_VIDEO_MALI_EGL_MALI)/endif(HAVE_VIDEO_MALI_EGL_FB)/' \
	"$build/src/cmake/sdlchecks.cmake"

cmake -S "$build/src" -B "$build/cmake" -G Ninja \
	-DCMAKE_BUILD_TYPE=Release \
	-DSDL_SHARED=ON -DSDL_STATIC=OFF -DSDL_TEST=OFF \
	-DSDL_MALI=ON -DSDL_KMSDRM=OFF -DSDL_X11=OFF -DSDL_WAYLAND=OFF \
	-DSDL_VIVANTE=OFF -DSDL_DIRECTFB=OFF -DSDL_RPI=OFF \
	-DSDL_OPENGL=ON -DSDL_OPENGLES=ON -DSDL_VULKAN=OFF \
	-DSDL_ALSA=ON -DSDL_ALSA_SHARED=ON \
	-DSDL_PULSEAUDIO=OFF -DSDL_PIPEWIRE=OFF -DSDL_JACK=OFF -DSDL_SNDIO=OFF
cmake --build "$build/cmake" --parallel

library=$(find "$build/cmake" -type f -name 'libSDL2-2.0.so.0.*' -print | head -n 1)
[ -n "$library" ] || { echo "SDL2 did not produce libSDL2-2.0.so.0" >&2; exit 1; }
strings "$library" | grep -q 'Mali EGL Video Driver'
cp "$library" "$output"
printf 'version=%s\nbackend=mali-fbdev\n' "$version" > "$output.meta"
