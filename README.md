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

## 使用说明

每个工具都有独立的安装和使用说明，请参考对应子目录中的 README 文件获取详细信息。