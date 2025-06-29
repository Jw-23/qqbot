# QQ机器人管理后台

这是一个基于 Actix Web 和 React + TypeScript 的 QQ 机器人管理后台系统。

## 功能特性

### 1. 学生管理
- ✅ 学生信息的增删改查
- ✅ 批量导入学生数据（CSV格式）
- ✅ 导出学生数据为CSV文件
- ✅ 支持分页查询

### 2. 成绩管理
- ✅ 成绩记录的增删改查
- ✅ 按学生查询成绩
- ✅ 支持多种成绩类别（Quiz-1, Quiz-2, Quiz-3, Quiz-4, Mid）
- ✅ 支持分页查询

### 3. 群发消息
- ✅ 按学号批量发送消息
- ✅ 支持文本输入学号（按行分割）
- ✅ 支持下拉选择学生
- ✅ 实时显示选中的学生

### 4. 系统配置
- ✅ 修改机器人基础配置
- ✅ 调整缓存配置
- ✅ 修改数据库连接配置  
- ✅ 调整LLM模型配置

## 技术栈

### 后端
- **Actix Web** - 高性能Web框架
- **SeaORM** - 现代化的ORM框架
- **MySQL** - 数据库
- **Serde** - 序列化/反序列化
- **CSV** - CSV文件处理

### 前端
- **React 18** - UI框架
- **TypeScript** - 类型安全
- **Ant Design 5** - UI组件库
- **Axios** - HTTP客户端
- **React Router** - 路由管理

## 安装和运行

### 前置要求
- Rust 1.70+
- Node.js 16+
- MySQL 8.0+

### 1. 克隆项目
```bash
git clone <repository-url>
cd qqbot-rs
```

### 2. 配置数据库
确保MySQL服务正在运行，并且数据库连接配置正确（在`config.dev.toml`中）。

### 3. 安装前端依赖
```bash
cd admin/frontend
npm install
```

### 4. 构建项目
```bash
# 从项目根目录运行
./admin/build.sh
```

### 5. 启动服务
```bash
cargo run -p admin
```

服务将在 `http://localhost:8080` 启动。

## 使用指南

### 学生管理

#### 导入学生数据
1. 准备CSV文件，格式如下：
```csv
学号,姓名,QQ号,群号
20210001,张三,1234567890,987654321
20210002,李四,1234567891,987654321
```

2. 在学生管理页面点击"导入CSV"按钮
3. 选择CSV文件并上传

#### 导出学生数据
1. 在学生管理页面点击"导出CSV"按钮
2. 浏览器将自动下载CSV文件

### 群发消息

#### 方式一：输入学号
1. 在"学号列表"文本框中输入学号，每行一个
2. 输入消息内容
3. 点击"发送消息"

#### 方式二：下拉选择
1. 在下拉选择框中选择学生
2. 输入消息内容
3. 点击"发送消息"

### 系统配置

#### 修改配置
1. 在系统配置页面修改相应配置项
2. 点击"保存配置"按钮
3. 重启机器人服务以应用新配置

## API文档

### 学生相关
- `GET /api/students` - 获取学生列表
- `POST /api/students` - 创建学生
- `GET /api/students/{id}` - 获取学生详情
- `PUT /api/students/{id}` - 更新学生信息
- `DELETE /api/students/{id}` - 删除学生
- `POST /api/students/import` - 批量导入学生
- `GET /api/students/export` - 导出学生数据
- `POST /api/students/bulk-message` - 群发消息

### 成绩相关
- `GET /api/grades` - 获取成绩列表
- `POST /api/grades` - 创建成绩记录
- `GET /api/grades/{id}` - 获取成绩详情
- `PUT /api/grades/{id}` - 更新成绩
- `DELETE /api/grades/{id}` - 删除成绩
- `GET /api/grades/student/{student_id}` - 获取学生成绩

### 配置相关
- `GET /api/config` - 获取系统配置
- `PUT /api/config` - 更新系统配置

## 开发指南

### 开发环境启动

#### 后端开发
```bash
# 启动后端服务器（开发模式）
cargo run -p admin
```

#### 前端开发
```bash
# 启动前端开发服务器
cd admin/frontend
npm start
```

前端开发服务器将在 `http://localhost:3000` 启动，API请求会自动代理到后端。

### 项目结构
```
admin/
├── src/
│   ├── main.rs           # 主程序入口
│   ├── handlers/         # API处理器
│   │   ├── student_handler.rs
│   │   ├── grade_handler.rs
│   │   └── config_handler.rs
│   ├── models/           # 数据模型
│   └── services/         # 业务逻辑
├── frontend/             # React前端
│   ├── src/
│   │   ├── pages/        # 页面组件
│   │   ├── services/     # API服务
│   │   └── types/        # TypeScript类型定义
│   └── public/
├── Cargo.toml
├── build.sh             # 构建脚本
└── README.md
```

## 注意事项

1. **数据库连接**：确保MySQL服务正在运行，并且配置文件中的数据库连接信息正确。

2. **权限控制**：当前版本没有实现用户认证，建议仅在内网环境使用。

3. **消息发送**：群发消息功能需要QQ机器人服务正在运行。

4. **配置修改**：修改系统配置后需要重启机器人服务以应用新配置。

5. **文件上传**：CSV导入功能对文件格式要求严格，请确保CSV文件格式正确。

## 故障排除

### 常见问题

1. **无法连接数据库**
   - 检查MySQL服务是否正在运行
   - 确认数据库连接配置是否正确
   - 检查数据库用户权限

2. **前端构建失败**
   - 确保Node.js版本>=16
   - 删除`node_modules`并重新安装依赖
   - 检查网络连接

3. **API请求失败**
   - 检查后端服务是否正在运行
   - 确认防火墙设置
   - 查看浏览器开发者工具的网络面板

4. **CSV导入失败**
   - 检查CSV文件格式是否正确
   - 确保文件编码为UTF-8
   - 检查数据是否有重复

## 贡献指南

欢迎提交Issue和Pull Request来改进项目。

## 许可证

本项目采用MIT许可证。
