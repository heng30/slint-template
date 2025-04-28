#!/bin/sh

if [ $# -ne 1 ] || [ "$1" == "-h" ] || [ "$1" == "--help" ] || [ "$1" == "-help" ]; then
    echo "Usage: $0 your-splash-picture.png"
    exit
fi

LOC=$(readlink -f "$0")
DIR=$(dirname "$LOC")
splash=$1

cp -f $splash $DIR/../android/res/drawable-hdpi/android12splash.png
cp -f $splash $DIR/../android/res/drawable-hdpi/splash.png
cp -f $splash $DIR/../android/res/drawable-mdpi/android12splash.png
cp -f $splash $DIR/../android/res/drawable-mdpi/splash.png
cp -f $splash $DIR/../android/res/drawable-xhdpi/android12splash.png
cp -f $splash $DIR/../android/res/drawable-xhdpi/splash.png
cp -f $splash $DIR/../android/res/drawable-xxhdpi/android12splash.png
cp -f $splash $DIR/../android/res/drawable-xxhdpi/splash.png
cp -f $splash $DIR/../android/res/drawable-xxxhdpi/android12splash.png
cp -f $splash $DIR/../android/res/drawable-xxxhdpi/splash.png

echo "finished"
