#!/bin/sh

# 检查是否提供了目录参数
if [ $# -ne 1 ]; then
    echo "Usage: $0 <target-project>"
    exit 1
fi

target_project="$1"
template_name="slint-template"

rsync -a --exclude='about.slint' ../$template_name/ui/base $target_project/ui/
rsync -a --exclude='brand.png' ../$template_name/ui/images $target_project/ui/
rsync -a ../lib/cutil $target_project/../lib/
rsync -a ../lib/sqldb $target_project/../lib/
rsync -a ../lib/pmacro $target_project/../lib/

