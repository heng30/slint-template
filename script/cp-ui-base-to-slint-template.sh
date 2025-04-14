#!/bin/sh

# 检查是否提供了目录参数
if [ $# -ne 1 ]; then
    echo "Usage: $0 <directory>"
    exit 1
fi

target_dir="$1"

cp -rf ../ui/base $target_dir/ui/
cp -rf ../ui/images $target_dir/ui/
cp -rf ../ui/theme.slint $target_dir/ui/
