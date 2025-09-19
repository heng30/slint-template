#!/bin/sh

# 检查是否提供了目录参数
if [ $# -ne 1 ]; then
    echo "Usage: $0 <directory>"
    exit 1
fi

target_dir="$1"

rsync -a --exclude='about.slint' ../slint-template/ui/base $target_dir/ui/
rsync -a --exclude='brand.png' ../slint-template/ui/images $target_dir/ui/
rsync -a ../slint-template/ui/theme.slint $target_dir/ui/
