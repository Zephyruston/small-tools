# Small Tools Collection

这是一个包含多个实用命令行工具的集合，旨在提升开发效率。

## 工具列表

### 1. 图像生成工具 (image_gen)

基于 ModelScope API 的图像生成命令行工具，支持 Python 和 Rust 两种实现。

主要特性：
- 支持多种图像尺寸
- 自动生成 hash 文件名保存图像
- 提供 Python 和 Rust 两个版本

[查看详细文档](./image_gen/README.md)

### 2. 工具发现器 (list_tools)

用于发现通过 Go、npm 和 Cargo 包管理器全局安装的工具，并生成结构化文档。

主要特性：
- 多包管理器支持
- 统一工具清单
- LLM 集成生成安装命令
- 自动化文档生成

[查看详细文档](./list_tools/README.md)

### 3. 简易翻译工具 (translate)

一个用 Rust 编写的命令行工具，用于中英翻译。

主要特性：
- 通过命令行进行中文到英文的翻译
- 优先使用本地词库进行翻译
- 支持接入 AI 翻译服务以处理本地词库未覆盖的词汇

[查看详细文档](./translate/README.md)

### 4. 环境变量加载工具 (load_env)

一个 CLI 工具，用于将 `.env` 文件中的环境变量加载到当前 shell 会话中。

主要特性：
- 从 `.env` 文件加载环境变量
- 将变量直接导出到当前 shell 会话
- 简单直观的命令行界面
- 支持多种 shell (bash, zsh, fish)

[查看详细文档](./load_env/README.md)

## 使用说明

每个工具都有独立的安装和使用说明，请参考对应子目录中的 README 文件获取详细信息。