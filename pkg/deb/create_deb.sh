#!/bin/sh

cp ../../target/release/slint-template ./package/usr/local/bin/
cp ../../ui/images/brand.png ./package/usr/share/icons/hicolor/256x256/apps/slint-template.png
cp ../../ui/images/brand.png ./package/usr/share/icons/hicolor/symbolic/apps/slint-template.png
cp ../../ui/images/brand.png ./package/usr/share/icons/hicolor/scalable/apps/slint-template.png

chmod a+x ./package/usr/local/bin/slint-template

dpkg-deb --build package

rm -f ./package/usr/local/bin/slint-template
rm -f ./package/usr/share/icons/hicolor/256x256/apps/slint-template.png
rm -f ./package/usr/share/icons/hicolor/symbolic/apps/slint-template.png
rm -f ./package/usr/share/icons/hicolor/scalable/apps/slint-template.png

mv -f package.deb slint-template.deb

exit $?
