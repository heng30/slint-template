#!/bin/sh

if [ $# -ne 1 ] || [ "$1" == "-h" ] || [ "$1" == "--help" ] || [ "$1" == "-help" ]; then
    echo "Usage: $0 your-icon.ico"
    exit
fi

LOC=$(readlink -f "$0")
DIR=$(dirname "$LOC")
icon=$1

cp -f $icon $DIR/../windows/icon.ico

echo "finished"
