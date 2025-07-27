# Pencil API - Hitokoto（一言）服务

这是一个基于 Rust Rocket 框架开发的现代化 Hitokoto（一言）API 服务，支持用户注册、内容提交、文集管理等完整功能。

## 功能特性

- **用户管理**: 用户注册、验证和信息管理
- **内容管理**: Hitokoto 提交、存储和随机获取
- **文集系统**: 用户可创建文集来组织 Hitokoto
- **三层架构**: 用户 → 文集 → Hitokoto 的完整层级结构
- **异步处理**: 基于 Tokio 的高性能异步 I/O
- **数据持久化**: JSON 文件存储，支持服务重启恢复
- **UUID 系统**: 所有实体使用 UUID 进行唯一标识
- **并发安全**: 使用 `tokio::sync::Mutex` 保证线程安全

## 技术栈

- **语言**: Rust
- **Web 框架**: Rocket 0.5
- **异步运行时**: Tokio
- **序列化**: Serde
- **随机数**: Rand
- **UUID**: UUID crate

## 项目结构

```
pencil_api/
├── src/
│   ├── main.rs          # 主程序和路由定义
│   ├── lib.rs           # 库模块导出
│   ├── storage.rs       # 数据存储和管理逻辑
│   ├── hitokoto.rs      # Hitokoto 数据结构
│   ├── user.rs          # 用户数据结构
│   └── collection.rs    # 文集数据结构
├── hitokoto.json        # Hitokoto 数据文件（自建）
├── user.json           # 用户数据文件（自建）
├── collection.json     # 文集数据文件（自建）
├── hitokoto.json       # Hitokoto 数据文件（自建）
├── Cargo.toml          # 依赖配置
├── Rocket.toml         # Rocket 服务器配置
└── API_USAGE.md        # API 使用文档
```

## 安装和运行

### 前置要求

- Rust 1.70+
- Cargo

### 编译和运行

```bash
# 克隆项目
git clone <repository-url>
cd pencil_api

# 编译项目
cargo build --release

# 运行服务
cargo run
```

服务将在 `http://0.0.0.0:8000` 启动。

## 详细文档

API 使用详情请参考 [API_USAGE.md](./API_USAGE.md)

## 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/new-feature`)
3. 提交更改 (`git commit -am 'Add new feature'`)
4. 推送分支 (`git push origin feature/new-feature`)
5. 创建 Pull Request

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 未来计划

- [ ] 添加数据库支持（PostgreSQL/MySQL）
- [ ] 实现用户认证和权限管理
- [ ] 完善测试覆盖率

---

如有问题或建议，请创建 Issue 或 Pull Request。
