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
sed -i "s/$old_project_name/$project_name/g" $OLD_PROJECT_DIR/Cargo.toml
sed -i "s/$old_android_project_name/$android_project_name/g" $OLD_PROJECT_DIR/Cargo.toml
sed -i "s/$old_project_name/$project_name/g" $ROOT_DIR/Cargo.toml
sed -i "s/$old_project_name/$project_name/g" $ROOT_DIR/Makefile
sed -i "s/$old_project_name/$project_name/g" $OLD_PROJECT_DIR/windows/version.h
sed -i "s/$old_project_name/$project_name/g" $OLD_PROJECT_DIR/ui/panel/setting/desktop.slint
sed -i "s/$old_project_name/$project_name/g" $OLD_PROJECT_DIR/ui/panel/setting/mobile.slint
sed -i "s/$old_desktop_project_name/$desktop_project_name/g" $OLD_PROJECT_DIR/src/desktop.rs
sed -i "s/$old_project_name/$project_name/g" $OLD_PROJECT_DIR/src/lib.rs
sed -i "s/$old_project_name/$project_name/g" $OLD_PROJECT_DIR/ui/base/about.slint
sed -i "s/$old_project_name/$project_name/g" $ROOT_DIR/tr-helper/build.rs
sed -i "s/$old_web_project_name/$project_name/g" $OLD_PROJECT_DIR/web/index.html
sed -i "s/$old_web_js_file_name/$web_js_file_name/g" $OLD_PROJECT_DIR/web/index.html

# init scripts
sed -i "s/$old_project_name/$project_name/g" $DIR/convert-brand-to-window-icon.sh
sed -i "s/$old_project_name/$project_name/g" $DIR/merge-to-slint-template-project.sh
sed -i "s/$old_project_name/$project_name/g" $DIR/replace-android-icon.sh
sed -i "s/$old_project_name/$project_name/g" $DIR/replace-android-splash.sh

mv $ROOT_DIR/$old_project_name $ROOT_DIR/$project_name

$ROOT_DIR/package/deb/init.sh $project_name
