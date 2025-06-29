#!/bin/bash

set -e

echo "🚀 启动QQ机器人管理后台..."

# 检查前端是否已构建
if [ ! -d "admin/frontend/build" ]; then
    echo "📦 前端未构建，正在构建..."
    cd admin/frontend
    npm run build
    cd ../..
fi

echo "🦀 启动后端服务器..."
echo "🌐 请在浏览器中打开: http://localhost:8080"
echo "📋 Ctrl+C 停止服务器"
echo ""

cargo run -p admin
