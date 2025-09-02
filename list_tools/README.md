# 包管理器工具发现与文档生成器

[![Python 3.10+](https://img.shields.io/badge/python-3.10+-blue.svg)](https://www.python.org/downloads/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

一个用于发现通过 Go、npm 和 Cargo 包管理器全局安装的工具，并生成包含安装命令的结构化文档的自动化工具。

## 功能特性

- 🎯 **多包管理器支持**: 扫描 Go、npm 和 Cargo 全局安装的工具
- 📋 **统一清单**: 聚合所有发现的工具到单一清单中
- 🤖 **LLM 集成**: 利用大语言模型分析并生成准确的安装命令
- 📝 **文档生成**: 自动生成结构化的 Markdown 文档
- ⚡ **高效发现**: 快速扫描并收集工具元数据
- 🔄 **自动更新**: 定期更新文档以保持信息同步

## 使用场景

- 维护开发环境工具清单
- 团队成员间共享工具配置
- 系统迁移时的工具备份
- 环境重建时的快速安装
- 工具依赖管理和文档化

## 快速开始

### 环境要求

- Python 3.10 或更高版本
- Go、npm 或 Cargo (至少安装其中一个)
- uv 包管理器 (推荐) 或 pip

### 安装

```bash
# 克隆项目
git clone <repository-url>
cd list_tools

# 使用 uv 安装依赖 (推荐)
uv sync

# 或使用 pip 安装依赖
pip install -r requirements.txt
```

### 基本使用

```bash
# 发现工具并生成文档
uv run main.py
```

生成的文档将保存在 `docs/tools.md` 文件中。

### 配置

要使用 LLM 功能生成安装命令，请创建一个 `.env` 文件：

```bash
cp .env.example .env
```

然后编辑 `.env` 文件，添加您的 API 密钥：

```bash
# OpenAI API 配置
OPENAI_API_KEY=your-openai-api-key
OPENAI_BASE_URL=https://api.openai.com/v1
OPENAI_MODEL=gpt-3.5-turbo
```

如果没有提供 API 密钥，工具将生成占位符安装命令。

## 架构设计

### 系统架构

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Go 扫描器     │    │   npm 扫描器    │    │ Cargo 扫描器    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │   工具聚合器     │
                    └─────────────────┘
                                 │
                    ┌─────────────────┐
                    │   LLM 分析器     │
                    └─────────────────┘
                                 │
                    ┌─────────────────┐
                    │   文档生成器     │
                    └─────────────────┘
```

### 数据流

1. **发现阶段**
   - 扫描每个包管理器的已安装工具
   - 收集元数据（名称、版本、路径、安装日期）
   - 将数据规范化为统一格式

2. **分析阶段**
   - 将工具信息发送给 LLM 进行命令生成
   - 验证和完善生成的命令
   - 处理边缘情况和未知工具

3. **文档阶段**
   - 生成结构化的 Markdown
   - 按包管理器和工具类别组织
   - 更新现有文档的变更

## 核心组件

### ToolDiscovery (工具发现器)

负责扫描系统中通过不同包管理器安装的工具：

- `discover_go_tools()`: 发现 Go 全局安装的包
- `discover_npm_tools()`: 发现 npm 全局安装的包
- `discover_cargo_tools()`: 发现 Cargo 全局安装的包
- `discover_all_tools()`: 聚合所有包管理器的工具

### LLMAnalyzer (LLM 分析器)

利用大语言模型分析工具信息并生成安装命令：

- `analyze_tools_from_markdown()`: 从 Markdown 文件读取工具信息并生成安装命令
- `update_documentation_with_commands()`: 更新文档中的安装命令

### DocumentationGenerator (文档生成器)

生成结构化的 Markdown 文档：

- `generate_markdown()`: 生成工具清单文档

## 生成的文档

工具运行后将生成以下文档：

- `docs/tools.md`: 包含所有发现工具的清单，按包管理器分组，包含工具名称、版本、描述和安装命令。

## 配置文件

- `pyproject.toml`: 项目依赖和元数据配置
- `.env`: API 密钥和 LLM 配置（需手动创建）
- `.env.example`: 环境变量配置示例

## 项目结构

```
├── main.py                 # 主程序入口
├── pyproject.toml          # 项目配置和依赖
├── README.md               # 项目说明文档
├── .env.example            # 环境变量示例
├── docs/                   # 生成的文档
│   └── tools.md            # 工具清单文档
├── spec/                   # 项目规格说明
│   └── 0001.md             # 核心功能规格说明
└── .qwen/                  # Qwen 扩展配置
    └── extensions/
        └── gen-doc/        # 文档生成扩展
```

## 开发指南

### 依赖管理

本项目使用 `uv` 进行依赖管理：

```bash
# 添加新依赖
uv add package-name

# 更新依赖
uv sync

# 导出 requirements.txt
uv pip compile pyproject.toml -o requirements.txt
```

### 代码结构

- `main.py`: 包含所有核心功能的实现
- 使用了 `pydantic` 进行数据建模
- 使用 `rich` 提供美观的终端输出
- 使用 `openai` 库集成 LLM 功能
- 使用 `dotenv` 管理环境变量

### 扩展开发

要添加对新包管理器的支持：

1. 在 `ToolDiscovery` 类中添加新的发现方法
2. 确保新方法返回 `ToolInfo` 对象列表
3. 在 `discover_all_tools()` 中调用新方法
4. 更新 LLM 提示以支持新包管理器

## 贡献指南

欢迎贡献代码！请遵循以下步骤：

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 联系方式

如有问题或建议，请创建 Issue 或联系项目维护者。