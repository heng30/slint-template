#!/bin/sh

if [ $# -ne 1 ] || [ "$1" == "-h" ] || [ "$1" == "--help" ] || [ "$1" == "-help" ]; then
    echo "Usage: $0 <splash.png>"
    exit
fi

template_name="slint-template"

cp -f "$1" ../$template_name/android/res/drawable-hdpi/android12splash.png
cp -f "$1" ../$template_name/android/res/drawable-hdpi/splash.png
cp -f "$1" ../$template_name/android/res/drawable-mdpi/android12splash.png
cp -f "$1" ../$template_name/android/res/drawable-mdpi/splash.png
cp -f "$1" ../$template_name/android/res/drawable-xhdpi/android12splash.png
cp -f "$1" ../$template_name/android/res/drawable-xhdpi/splash.png
cp -f "$1" ../$template_name/android/res/drawable-xxhdpi/android12splash.png
cp -f "$1" ../$template_name/android/res/drawable-xxhdpi/splash.png
cp -f "$1" ../$template_name/android/res/drawable-xxxhdpi/android12splash.png
cp -f "$1" ../$template_name/android/res/drawable-xxxhdpi/splash.png
