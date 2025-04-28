#!/bin/sh

if [ $# -ne 1 ] || [ "$1" == "-h" ] || [ "$1" == "--help" ] || [ "$1" == "-help" ]; then
    echo "Usage: $0 your-icon.png"
    exit
fi

LOC=$(readlink -f "$0")
DIR=$(dirname "$LOC")
icon=$1

cp -f $icon $DIR/../android/res/mipmap-hdpi/ic_launcher.png
cp -f $icon $DIR/../android/res/mipmap-ldpi/ic_launcher.png
cp -f $icon $DIR/../android/res/mipmap-mdpi/ic_launcher.png
cp -f $icon $DIR/../android/res/mipmap-xhdpi/ic_launcher.png
cp -f $icon $DIR/../android/res/mipmap-xxhdpi/ic_launcher.png
cp -f $icon $DIR/../android/res/mipmap-xxhdpi/ic_launcher.png

echo "finished"
