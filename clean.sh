#!/bin/bash

echo "开始清理构建文件..."

# 清理 Rust 构建产物
echo "清理 Rust 构建文件..."
cd src-tauri && cargo clean
cd ..

# 清理前端构建产物
echo "清理前端构建文件..."
rm -rf dist
rm -rf node_modules

# 清理其他可能的临时文件
echo "清理临时文件..."
find . -name ".DS_Store" -delete
find . -name "*.log" -delete

echo "清理完成！"