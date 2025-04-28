# 使用大语言模型进行翻译的prompt
# 按行翻译上面的英文。大小写敏感，即使相同的单词也要进行翻译。输出格式如下：
# ```
# ("英文", "翻译的中文"),
# ```

#!/bin/sh

# 检查是否提供了目录参数
if [ $# -ne 1 ]; then
    echo "Usage: $0 <directory>"
    exit 1
fi

target_dir="$1"
temp_file=$(mktemp)

# 查找所有slint和rs文件，跳过target和.git目录
find "$target_dir" -type f \( -name "*.slint" -o -name "*.rs" \) \
    -not -path "*/.git/*" \
    -not -path "*/target/*" \
    -print0 | while IFS= read -r -d $'\0' file; do

    # 提取Logic.tr("contents")和tr("contents")中的内容并存入临时文件
    grep -o -E '(Logic\.tr\("([^"\\]|\\.)*"\)|tr\("([^"\\]|\\.)*"\))' "$file" | \
    sed -E 's/(Logic\.tr\("|tr\(")([^"]*)("\))/\2/' >> "$temp_file"
done

# 对临时文件内容进行排序和去重
sort -u "$temp_file" | while IFS= read -r line; do
    echo "$line"
done

# 删除临时文件
rm -f "$temp_file"
