# Generate by AI

import os
import re


def rename_svg_files(directory):
    """
    遍历指定目录，重命名SVG文件

    规则：
    1. 包含 `-light.svg` 或 `-fill.svg` 的文件名保持不变
    2. 其他SVG文件重命名为 `原文件名-light.svg`
    """

    # 支持的SVG文件扩展名
    svg_extensions = (".svg", ".SVG")

    # 需要保留的特殊文件名模式
    preserved_patterns = ["-light.svg", "-fill.svg"]

    # 遍历目录中的所有文件
    for filename in os.listdir(directory):
        file_path = os.path.join(directory, filename)

        # 只处理文件，跳过目录
        if not os.path.isfile(file_path):
            continue

        # 只处理SVG文件
        if not filename.lower().endswith(svg_extensions):
            continue

        # 检查是否需要保留原文件名
        should_preserve = any(pattern in filename for pattern in preserved_patterns)

        if should_preserve:
            print(f"保留文件: {filename}")
            continue

        # 构建新文件名
        # 移除原有的.svg扩展名
        base_name = os.path.splitext(filename)[0]
        new_filename = f"{base_name}-light.svg"
        new_file_path = os.path.join(directory, new_filename)

        # 检查新文件名是否已存在
        if os.path.exists(new_file_path):
            print(f"警告: 文件 {new_filename} 已存在，跳过重命名 {filename}")
            continue

        # 重命名文件
        try:
            os.rename(file_path, new_file_path)
            print(f"重命名: {filename} -> {new_filename}")
        except Exception as e:
            print(f"错误: 无法重命名 {filename}: {str(e)}")


def main():
    # 获取要遍历的目录（默认为当前目录）
    target_directory = input("请输入要遍历的目录路径（默认为当前目录）: ").strip()

    if not target_directory:
        target_directory = os.getcwd()

    # 检查目录是否存在
    if not os.path.isdir(target_directory):
        print(f"错误: 目录 '{target_directory}' 不存在")
        return

    print(f"开始处理目录: {target_directory}")
    print("-" * 50)

    rename_svg_files(target_directory)

    print("-" * 50)
    print("处理完成")


if __name__ == "__main__":
    main()
