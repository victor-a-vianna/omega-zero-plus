#!/bin/sh
set -eu

[ $# -eq 4 ] || exit 2
jar="$1"
work="$2"
out="$3"
font_libs="$4"
runtime="$work/freej2me_plus_runtime/linux-arm64"

rm -rf "$work"
mkdir -p "$(dirname "$runtime")"
"${JAVA_HOME:?}/bin/jlink" \
    --module-path "$JAVA_HOME/jmods" \
    --add-modules java.base,java.desktop \
    --compress=2 \
    --no-header-files \
    --no-man-pages \
    --strip-debug \
    --output "$runtime"
cp "$jar" "$runtime/bin/freej2me_plus-lr.jar"
mkdir -p "$runtime/conf/fonts" "$runtime/lib/fonts"
cp cores/freej2me_plus/fonts.conf "$runtime/conf/fonts/fonts.conf"
cp crates/omega-ui/assets/PixelCode-Regular.ttf "$runtime/lib/fonts/PixelCode-Regular.ttf"
mkdir -p "$runtime/lib/fontconfig"
cp "$font_libs"/* "$runtime/lib/fontconfig/"
mkdir -p "$(dirname "$out")"
# BASEOS is installed on FAT, which cannot create the JDK's symbolic links.
# Store their targets as regular files so first-boot extraction works there.
tar --hard-dereference -chzf "$out" -C "$work" freej2me_plus_runtime
