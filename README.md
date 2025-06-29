# qqbot

# 支持命令

## 查询命令，用于查询成绩信息


一个基于 Rust 的可扩展 QQ 机器人.

## 目录结构说明

- migration/           数据库迁移与管理模块
- plugins/             插件目录（如 apply-request、cmd-reply）
- qqbot-cmd/           命令行入口，负责启动机器人
- qqbot-core/          核心功能与业务逻辑
- qqbot-derive/        自定义派生宏
- admin/               Web管理后台（React + Rust）
- config.dev.toml      配置文件示例
- README.md            项目说明文档

## 主要功能

- 支持多种命令（如绑定、查询等）
- 插件化架构，易于扩展
- 异步数据库访问（SeaORM）
- 命令行参数解析（clap）
- 数据库迁移与版本管理
- **Web管理后台**：学生管理、成绩管理、群发消息、系统配置
- 良好的测试覆盖

## 快速开始

### 启动QQ机器人

1. **配置数据库与环境**
   - 复制 `config.dev.toml` 为 `config.toml` 并根据实际情况修改数据库等配置。

2. **运行sea-orm数据库迁移**
   ```sh
   sea-orm-cli migrate up --database-url "DATABASE_URL"
   ```

3. **编译并运行主程序**
   ```sh
   cd ../qqbot-cmd
   cargo run
   ```

### 启动Web管理后台

1. **构建并启动管理后台**
   ```sh
   ./admin/start.sh
   ```

2. **访问管理界面**
   - 打开浏览器访问 `http://localhost:8080`
   - 功能包括：
     - 📊 学生信息管理（增删改查、导入导出）
     - 📝 成绩管理（按学生查询、批量管理）
     - 📢 群发消息（按学号批量发送）
     - ⚙️ 系统配置（修改机器人配置文件）

3. **演示数据**
   ```sh
   ./admin/create_demo_data.sh  # 创建演示用的学生数据
   ```

## 管理后台使用指南

### 学生管理
- **导入学生**：支持CSV格式批量导入
- **导出数据**：一键导出所有学生信息
- **在线编辑**：支持单个学生信息的在线修改

### 成绩管理
- **多类别成绩**：支持Quiz-1~4、Mid等多种成绩类别
- **关联查询**：可按学生查询所有成绩记录
- **快速录入**：支持成绩的快速录入和修改

### 群发消息
- **按学号发送**：输入学号列表，支持按行分割
- **选择式发送**：通过下拉选择目标学生
- **实时预览**：显示选中的学生列表

### 系统配置
- **在线配置**：Web界面修改config.dev.toml
- **分类管理**：按基础、缓存、数据库、LLM分类配置
- **即时生效**：保存后重启服务即可应用新配置

## 插件开发

- 使用kovi-cli `cargo kovi add hello-world`
- 在 `qqbot-core/cmd` 中编写并且在`qqbot/cmd/mod`注册插件

## 依赖

- Rust 1.70+
- SeaORM
- clap
- tokio
- 其他依赖详见各子 crate 的 `Cargo.toml`

## 贡献

欢迎提交 issue 和 PR！请遵循 [Rust 代码规范](https://doc.rust-lang.org/1.0.0/style/)。

## License

MIT
