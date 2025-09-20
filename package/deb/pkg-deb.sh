#!/bin/bash

LOC=$(readlink -f "$0")
DIR=$(dirname "$LOC")
ROOT_DIR="$DIR/../.."

app_name="slint-template"
icon_name="brand.png"
icon_dir="$ROOT_DIR/slint-template/ui/images/png"
bin_dir="$DIR/package/usr/local/bin"
dst_icon_name="${app_name}.png"
dst_icon_name_svg="${app_name}.svg"
sizes=(16x16 22x22 24x24 32x32 36x36 48x48 64x64 72x72 96x96 128x128 192x192 256x256 512x512)

mkdir -p ${bin_dir}
cp $ROOT_DIR/target/release/${app_name} ${bin_dir}
chmod a+x ${bin_dir}/${app_name}

for size in "${sizes[@]}"; do
    mkdir -p $DIR/package/usr/share/icons/hicolor/${size}/apps
    convert "${icon_dir}/${icon_name}" -resize "$size" -background none -gravity center -extent "$size" "$DIR/package/usr/share/icons/hicolor/${size}/apps/${dst_icon_name}"
done

dpkg-deb --build package ${app_name}.deb

rm -f ${bin_dir}/${app_name}

for size in "${sizes[@]}"; do
    rm -f $DIR/package/usr/share/icons/hicolor/${size}/apps/${dst_icon_name}
done

exit $?
