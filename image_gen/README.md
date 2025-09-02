# ModelScope 图像生成 CLI 工具

这是一个通过 ModelScope API 生成图像的命令行工具，支持 Python 和 Rust 两种实现版本。

## 功能特点

1. 通过 ModelScope 在线 API 生成图像
2. 支持多种图像尺寸（根据 ModelScope API 文档，Qwen-Image 支持范围[64x64,1664x1664]）：
   - 1328x1328 (1:1 正方形，默认)
   - 1664x928 (16:9 横屏)
   - 928x1664 (9:16 竖屏)
   - 1472x1140 (4:3 标准横屏)
   - 1140x1472 (3:4 标准竖屏)
3. 自动生成 hash 文件名保存图像

## 目录结构

该项目包含两个版本的实现：

- `image_gen.py`：Python 版本实现
- `image_gen_rs/`：Rust 版本实现

## 安装和配置

### Python 版本

1. 确保已安装 Python 3.10 或更高版本
2. 安装依赖：
   ```bash
   uv sync
   ```
3. 配置环境变量：
   复制 `.env.example` 文件并重命名为 `.env`，然后填入你的 ModelScope API 密钥：
   ```bash
   cp .env.example .env
   # 编辑 .env 文件，填入你的 API 密钥
   ```

### Rust 版本

1. 安装 Rust（如果尚未安装）：
   访问 [https://www.rust-lang.org/](https://www.rust-lang.org/) 并按照说明安装。

2. 构建项目：
   ```bash
   cd image_gen_rs
   cargo build --release
   ```

3. 或者安装到系统路径：
   ```bash
   cd image_gen_rs
   cargo install --path .
   ```
   这会将程序安装到 `~/.cargo/bin/` 目录下。

4. 配置环境变量：
   在运行命令的目录下创建 `.env` 文件：
   ```bash
   cp .env.example .env
   # 编辑 .env 文件，填入你的 API 密钥
   ```

### 环境变量配置

在 `.env` 文件中需要配置以下环境变量：

```env
API_BASE_URL=https://api-inference.modelscope.cn/
API_KEY=your_api_key_here
MODEL_NAME=Qwen/Qwen-Image
```

## 使用方法

### Python 版本

#### 基本用法

```bash
uv run image_gen.py "一个美丽的风景画"
```

#### 指定图像尺寸

```bash
uv run image_gen.py "一个美丽的风景画" --size 1328x1328
```

#### 指定输出文件名

```bash
uv run image_gen.py "一个美丽的风景画" --output my_image.png
```

#### 完整示例

```bash
# 生成1:1正方形图像
uv run image_gen.py "吉卜力风格的小镇，有风车和樱花树" --size 1328x1328

# 生成16:9横屏图像并指定文件名
uv run image_gen.py "未来城市的夜景，霓虹灯闪烁" --size 1664x928 --output city.png

# 生成9:16竖屏图像
uv run image_gen.py "一只可爱的猫咪穿着太空服" --size 928x1664
```

### Rust 版本

#### 基本用法

```bash
# 如果已安装到 cargo bin 目录
image_gen "一个美丽的风景画"

# 或者直接运行项目
cd image_gen_rs
cargo run -- "一个美丽的风景画"
```

#### 指定图像尺寸

```bash
image_gen "一个美丽的风景画" --size 1328x1328
```

#### 指定输出文件名

```bash
image_gen "一个美丽的风景画" --output my_image.png
```

#### 完整示例

```bash
# 生成1:1正方形图像
image_gen "吉卜力风格的小镇，有风车和樱花树" --size 1328x1328

# 生成16:9横屏图像并指定文件名
image_gen "未来城市的夜景，霓虹灯闪烁" --size 1664x928 --output city.png

# 生成9:16竖屏图像
image_gen "一只可爱的猫咪穿着太空服" --size 928x1664
```

## 宽高比说明

| 尺寸      | 宽高比 | 用途说明                           |
| --------- | ------ | ---------------------------------- |
| 1328x1328 | 1:1    | 正方形，适合头像、图标等           |
| 1664x928  | 16:9   | 横屏，适合桌面壁纸、横幅等         |
| 928x1664  | 9:16   | 竖屏，适合手机壁纸、社交媒体图片等 |
| 1472x1140 | 4:3    | 标准横屏，适合传统显示器显示       |
| 1140x1472 | 3:4    | 标准竖屏，适合特定应用场景         |

## 注意事项

1. 需要配置 ModelScope Token 才能使用 API
2. 图像生成通常需要几十秒时间
3. 生成的图像将保存在当前目录下
4. 使用 `uv run image_gen.py --help` (Python版本) 或 `image_gen --help` (Rust版本) 命令可以查看详细的帮助信息
