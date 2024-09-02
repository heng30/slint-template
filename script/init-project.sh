#!/bin/sh

if [ $# -ne 1 ] || [ "$1" == "-h" ] || [ "$1" == "--help" ] || [ "$1" == "-help" ]; then
    echo "Usage: $0 your-project-name"
    exit
fi


LOC=$(readlink -f "$0")
DIR=$(dirname "$LOC")

old_project_name="slint-template"
old_android_project_name="slint_template"
old_desktop_project_name=$old_android_project_name
old_web_project_name="Slint Template"
old_web_js_file_name=$old_android_project_name

project_name=$1
android_project_name=$(echo "$project_name" | tr '-' '_')
desktop_project_name=$android_project_name
web_js_file_name=$android_project_name

sed "s/$old_project_name/$project_name/g" $DIR/../Cargo.toml > $DIR/cache.txt
cp -f $DIR/cache.txt $DIR/../Cargo.toml

sed "s/$old_android_project_name/$android_project_name/g" $DIR/../Cargo.toml > $DIR/cache.txt
cp -f $DIR/cache.txt $DIR/../Cargo.toml

sed "s/$old_project_name/$project_name/g" $DIR/../Makefile > $DIR/cache.txt
cp -f $DIR/cache.txt $DIR/../Makefile

sed "s/$old_project_name/$project_name/g" $DIR/../windows/version.h > $DIR/cache.txt
cp -f $DIR/cache.txt $DIR/../windows/version.h

sed "s/$old_desktop_project_name/$desktop_project_name/g" $DIR/../src/desktop.rs > $DIR/cache.txt
cp -f $DIR/cache.txt $DIR/../src/desktop.rs

sed "s/$old_project_name/$project_name/g" $DIR/../ui/base/about.slint > $DIR/cache.txt
cp -f $DIR/cache.txt $DIR/../ui/base/about.slint

sed "s/$old_web_project_name/$project_name/g" $DIR/../web/index.html > $DIR/cache.txt
cp -f $DIR/cache.txt $DIR/../web/index.html

sed "s/$old_web_js_file_name/$web_js_file_name/g" $DIR/../web/index.html > $DIR/cache.txt
cp -f $DIR/cache.txt $DIR/../web/index.html

rm -f $DIR/cache.txt

echo "finished"
