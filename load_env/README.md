# load_env

一个 CLI 工具，用于将 `.env` 文件中的环境变量加载到当前 shell 会话中。

## 功能特性

- 从 `.env` 文件加载环境变量
- 将变量直接导出到当前 shell 会话
- 简单直观的命令行界面
- 对缺失或格式错误的文件进行适当的错误处理
- 支持多种 shell (bash, zsh, fish)

## 安装

要安装此工具，您需要在系统上安装 Rust 和 Cargo。

### 方法一：使用 Cargo 安装

```bash
# 直接从项目路径安装（您需要在项目根目录下执行）
cargo install --path .
```

## 使用方法

基本用法：

```bash
load_env <.env文件路径>
```

要实际设置当前 shell 中的环境变量，您需要根据使用的 shell 选择适当的方法来评估输出：

### Bash/Zsh

```bash
# 使用 source 和进程替换
source <(load_env .env)
```

### Fish

```bash
# 使用管道和 source
load_env .env | source
```

示例 `.env` 文件：

```env
DATABASE_URL=postgres://user:password@localhost:5432/mydb
API_KEY=1234567890abcdef
DEBUG=true
PORT=3000
```

## 工作原理

`load_env` 工具读取指定的 `.env` 文件，并输出 shell export 命令，这些命令可以被评估以在当前 shell 中设置这些变量。它不直接修改父 shell，而是提供必要的命令供 shell 评估和设置变量。

## 运行测试

要运行单元测试：

```bash
cargo test
```

## 贡献

欢迎贡献！请随时提交 Pull Request。

## 许可证

本项目采用 MIT 许可证 - 有关详细信息，请参阅 LICENSE 文件。
