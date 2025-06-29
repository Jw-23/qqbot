#!/bin/bash

set -e

echo "🔧 构建QQ机器人管理后台..."

# 构建前端
echo "📦 构建前端..."
cd admin/frontend
npm run build
cd ../..

echo "🦀 构建后端..."
cargo build --release -p admin

echo "✅ 构建完成！"
echo "🚀 启动服务器: cargo run -p admin"
echo "🌐 访问地址: http://localhost:8080"
