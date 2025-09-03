# 简易翻译 CLI 工具

这是一个用 Rust 编写的命令行工具，用于中英翻译。

## 功能

- 通过命令行进行中文到英文的翻译
- 优先使用本地词库进行翻译
- 支持接入 AI 翻译服务以处理本地词库未覆盖的词汇

## 安装

### 从源码安装

1. 克隆项目到本地：
```bash
git clone <repository-url>
cd translate
```

2. 下载词典数据：
   - 访问 [webextension_english_chinese_dictionary](https://github.com/program-in-chinese/webextension_english_chinese_dictionary)
   - 下载词典数据并放入 `~/.translate/dict` 目录中

3. 安装工具：
```bash
cargo install --path .
```

### 使用预编译版本

暂无预编译版本，建议从源码安装。

## 使用方法

```bash
# 翻译本地词库中的词汇
translate "制造或修理钟表者"

# 使用 AI 翻译服务翻译不在本地词库中的词汇
translate "不存在的词" --ai
```

## 环境变量配置

要使用 AI 翻译功能，需要配置相应的环境变量。可以创建一个 `.env` 文件（参考 `.env.example`）并设置以下变量：

```bash
# API 密钥 (必需)
OPENAI_API_KEY=your_api_key_here

# API 基础 URL (可选，默认为 OpenAI 的 API)
OPENAI_BASE_URL=https://api.openai.com/v1/

# 使用的模型 (可选，默认为 gpt-3.5-turbo)
OPENAI_MODEL=gpt-3.5-turbo
```

支持的 AI 服务配置示例：

### GLM (智谱 AI)
```bash
OPENAI_BASE_URL=https://open.bigmodel.cn/api/paas/v4/
OPENAI_API_KEY=your_glm_api_key_here
OPENAI_MODEL=glm-4.5
```

### DeepSeek
```bash
OPENAI_BASE_URL=https://api.deepseek.com/v1/
OPENAI_API_KEY=your_deepseek_api_key_here
OPENAI_MODEL=deepseek-chat
```

## 本地词库

本地词库文件位于 `~/.translate/dict` 目录下。该目录包含多个 JSON 文件，每个文件都是一个词典。程序会自动加载该目录下的所有词典文件。

## 项目结构

- `src/main.rs`: 主程序文件
- `Cargo.toml`: 项目依赖配置文件
- `.env.example`: 环境变量配置示例文件

## 未来计划

- 支持更多语言
- 支持词库的扩展和更新
- 提供更友好的命令行交互