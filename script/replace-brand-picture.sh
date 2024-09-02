#!/bin/sh

if [ $# -ne 1 ] || [ "$1" == "-h" ] || [ "$1" == "--help" ] || [ "$1" == "-help" ]; then
    echo "Usage: $0 your-brand.png"
    exit
fi

LOC=$(readlink -f "$0")
DIR=$(dirname "$LOC")
picture=$1

cp -f $picture $DIR/../ui/images/brand.png

echo "finished"
