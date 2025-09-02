# ModelScope 图像生成 CLI 工具 (Rust 版本)

这是一个使用 Rust 编写的命令行工具，通过 ModelScope API 生成图像。

## 功能特点

1. 通过 ModelScope 在线 API 生成图像
2. 支持多种图像尺寸（根据 ModelScope API 文档，Qwen-Image 支持范围[64x64,1664x1664]）：
   - 1328x1328 (1:1 正方形，默认)
   - 1664x928 (16:9 横屏)
   - 928x1664 (9:16 竖屏)
   - 1472x1140 (4:3 标准横屏)
   - 1140x1472 (3:4 标准竖屏)
3. 自动生成 hash 文件名保存图像

## 安装

### 安装 Rust

如果你还没有安装 Rust，请访问 [https://www.rust-lang.org/](https://www.rust-lang.org/) 并按照说明安装。

### 安装 image_gen 工具

要将 `image_gen` 安装到本地的 `~/.cargo/bin` 目录，可以使用以下命令：

```bash
cd image_gen
cargo install --path .
```

这将会把程序编译并安装到 `~/.cargo/bin/` 目录下，之后你就可以在任何地方直接使用 `image_gen` 命令了。

注意：在运行程序之前，你需要确保在运行命令的目录下有正确的 `.env` 文件，或者设置相应的环境变量。

## 构建和运行

### 构建项目

```bash
cd image_gen
cargo build --release
```

### 运行项目

```bash
cd image_gen
cargo run -- "一个美丽的风景画"
```

## 使用方法

### 基本用法

```bash
# 如果已安装到 cargo bin 目录
image_gen "一个美丽的风景画"

# 或者直接运行项目
cargo run -- "一个美丽的风景画"
```

### 指定图像尺寸

```bash
image_gen "一个美丽的风景画" --size 1328x1328
```

### 指定输出文件名

```bash
image_gen "一个美丽的风景画" --output my_image.png
```

### 完整示例

```bash
# 生成1:1正方形图像
image_gen "吉卜力风格的小镇，有风车和樱花树" --size 1328x1328

# 生成16:9横屏图像并指定文件名
image_gen "未来城市的夜景，霓虹灯闪烁" --size 1664x928 --output city.png

# 生成9:16竖屏图像
image_gen "一只可爱的猫咪穿着太空服" --size 928x1664
```

## 配置

在运行程序之前，需要创建一个 `.env` 文件并配置以下环境变量：

```env
API_BASE_URL=https://api-inference.modelscope.cn/
API_KEY=your_api_key_here
MODEL_NAME=Qwen/Qwen-Image
```

你可以复制 `.env.example` 文件并修改其中的值：

```bash
cp .env.example .env
# 然后编辑 .env 文件填入你的 API 密钥
```

如果使用 `cargo install` 安装了程序，你需要确保在运行 `image_gen` 命令的目录下有 `.env` 文件。

## 宽高比说明

| 尺寸      | 宽高比 | 用途说明                           |
| --------- | ------ | ---------------------------------- |
| 1328x1328 | 1:1    | 正方形，适合头像、图标等           |
| 1664x928  | 16:9   | 横屏，适合桌面壁纸、横幅等         |
| 928x1664  | 9:16   | 竖屏，适合手机壁纸、社交媒体图片等 |
| 1472x1140 | 4:3    | 标准横屏，适合传统显示器显示       |
| 1140x1472 | 3:4    | 标准竖屏，适合特定应用场景         |

使用 `image_gen --help` 命令可以查看详细的帮助信息。

## 注意事项

1. 需要配置 ModelScope Token 才能使用 API
2. 图像生成通常需要几十秒时间
3. 生成的图像将保存在当前目录下