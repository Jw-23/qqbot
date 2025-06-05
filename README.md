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
- config.dev.toml      配置文件示例
- README.md            项目说明文档

## 主要功能

- 支持多种命令（如绑定、查询等）
- 插件化架构，易于扩展
- 异步数据库访问（SeaORM）
- 命令行参数解析（clap）
- 数据库迁移与版本管理
- 良好的测试覆盖

## 快速开始


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
