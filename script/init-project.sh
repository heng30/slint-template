#!/bin/sh

if [ $# -ne 1 ] || [ "$1" == "-h" ] || [ "$1" == "--help" ] || [ "$1" == "-help" ]; then
    echo "Usage: $0 your-project-name"
    exit
fi

LOC=$(readlink -f "$0")
DIR=$(dirname "$LOC")
ROOT_DIR=$DIR/..
OLD_PROJECT_DIR=$ROOT_DIR/slint-template

old_project_name="slint-template"
old_android_project_name="slint_template"
old_desktop_project_name=$old_android_project_name
old_web_project_name="Slint Template"
old_web_js_file_name=$old_android_project_name

project_name=$1
android_project_name=$(echo "$project_name" | tr '-' '_')
desktop_project_name=$android_project_name
web_js_file_name=$android_project_name

# init src files
sed "s/$old_project_name/$project_name/g" $OLD_PROJECT_DIR/Cargo.toml > $DIR/cache.txt
cp -f $DIR/cache.txt $OLD_PROJECT_DIR/Cargo.toml

sed "s/$old_android_project_name/$android_project_name/g" $OLD_PROJECT_DIR/Cargo.toml > $DIR/cache.txt
cp -f $DIR/cache.txt $OLD_PROJECT_DIR/Cargo.toml

sed "s/$old_project_name/$project_name/g" $ROOT_DIR/Cargo.toml > $DIR/cache.txt
cp -f $DIR/cache.txt $ROOT_DIR/Cargo.toml

sed "s/$old_project_name/$project_name/g" $ROOT_DIR/Makefile > $DIR/cache.txt
cp -f $DIR/cache.txt $ROOT_DIR/Makefile

sed "s/$old_project_name/$project_name/g" $OLD_PROJECT_DIR/windows/version.h > $DIR/cache.txt
cp -f $DIR/cache.txt $OLD_PROJECT_DIR/windows/version.h

sed "s/$old_desktop_project_name/$desktop_project_name/g" $OLD_PROJECT_DIR/src/desktop.rs > $DIR/cache.txt
cp -f $DIR/cache.txt $OLD_PROJECT_DIR/src/desktop.rs

sed "s/$old_project_name/$project_name/g" $OLD_PROJECT_DIR/src/lib.rs > $DIR/cache.txt
cp -f $DIR/cache.txt $OLD_PROJECT_DIR/src/lib.rs

sed "s/$old_project_name/$project_name/g" $OLD_PROJECT_DIR/ui/base/about.slint > $DIR/cache.txt
cp -f $DIR/cache.txt $OLD_PROJECT_DIR/ui/base/about.slint

sed "s/$old_project_name/$project_name/g" $ROOT_DIR/tr-helper/build.rs > $DIR/cache.txt
cp -f $DIR/cache.txt $ROOT_DIR/tr-helper/build.rs

sed "s/$old_web_project_name/$project_name/g" $OLD_PROJECT_DIR/web/index.html > $DIR/cache.txt
cp -f $DIR/cache.txt $OLD_PROJECT_DIR/web/index.html

sed "s/$old_web_js_file_name/$web_js_file_name/g" $OLD_PROJECT_DIR/web/index.html > $DIR/cache.txt
cp -f $DIR/cache.txt $OLD_PROJECT_DIR/web/index.html

# init scripts
sed "s/$old_project_name/$project_name/g" $DIR/convert-brand-to-window-icon.sh > $DIR/cache.txt
cp -f $DIR/cache.txt $DIR/convert-brand-to-window-icon.sh

sed "s/$old_project_name/$project_name/g" $DIR/merge-to-slint-template-project.sh > $DIR/cache.txt
cp -f $DIR/cache.txt $DIR/merge-to-slint-template-project.sh

sed "s/$old_project_name/$project_name/g" $DIR/replace-android-icon.sh > $DIR/cache.txt
cp -f $DIR/cache.txt $DIR/replace-android-icon.sh

sed "s/$old_project_name/$project_name/g" $DIR/replace-android-splash.sh > $DIR/cache.txt
cp -f $DIR/cache.txt $DIR/replace-android-splash.sh

rm -f $DIR/cache.txt

mv $ROOT_DIR/$old_project_name $ROOT_DIR/$project_name

$ROOT_DIR/package/deb/init.sh $project_name
