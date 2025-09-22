#!/bin/sh

if [ $# -ne 1 ] || [ "$1" == "-h" ] || [ "$1" == "--help" ] || [ "$1" == "-help" ]; then
    echo "Usage: $0 <icon.png>"
    exit
fi

template_name="slint-template"

cp -f "$1" ../$template_name/android/res/mipmap-hdpi/ic_launcher.png
cp -f "$1" ../$template_name/android/res/mipmap-ldpi/ic_launcher.png
cp -f "$1" ../$template_name/android/res/mipmap-mdpi/ic_launcher.png
cp -f "$1" ../$template_name/android/res/mipmap-xhdpi/ic_launcher.png
cp -f "$1" ../$template_name/android/res/mipmap-xxhdpi/ic_launcher.png
cp -f "$1" ../$template_name/android/res/mipmap-xxhdpi/ic_launcher.png
