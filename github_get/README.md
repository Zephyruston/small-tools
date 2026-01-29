# github_get - GitHub 文件下载工具

将 GitHub URL 转换为可直接下载的 URL，支持多种下载方式。

## 安装

```bash
# 方式一：直接复制二进制文件
cp github_get ~/.local/bin/

# 方式二：添加到 PATH 的任意目录
```

确保 `github_get` 具有执行权限：
```bash
chmod +x github_get
```

## 使用方法

```bash
github_get [-h] [-o OUTPUT] [-c] [-d] [-e] url
```

### 参数说明

| 参数 | 说明 |
|------|------|
| `url` | GitHub 文件 URL（必需） |
| `-h, --help` | 显示帮助信息 |
| `-o, --output OUTPUT` | 指定输出文件名 |
| `-c, --curl` | 生成 curl 下载命令而不是 wget |
| `-d, --direct` | 直接显示转换后的原始 URL |
| `-e, --execute` | 直接执行下载命令 |

## 使用示例

### 1. 生成 wget 下载命令（默认）

```bash
github_get https://github.com/volcengine/volc-sdk-golang/blob/main/example/tls/demo_producer.go
```

输出：
```bash
wget -O demo_producer.go https://raw.githubusercontent.com/volcengine/volc-sdk-golang/main/example/tls/demo_producer.go
```

### 2. 指定输出文件名

```bash
github_get -o custom_name.go https://github.com/volcengine/volc-sdk-golang/blob/main/example/tls/demo_producer.go
```

### 3. 使用 curl 替代 wget

```bash
github_get -c https://github.com/volcengine/volc-sdk-golang/blob/main/example/tls/demo_producer.go
```

输出：
```bash
curl -L -o demo_producer.go https://raw.githubusercontent.com/volcengine/volc-sdk-golang/main/example/tls/demo_producer.go
```

### 4. 直接显示原始 URL

```bash
github_get -d https://github.com/volcengine/volc-sdk-golang/blob/main/example/tls/demo_producer.go
```

输出：
```bash
https://raw.githubusercontent.com/volcengine/volc-sdk-golang/main/example/tls/demo_producer.go
```

### 5. 直接执行下载

```bash
github_get -e https://github.com/volcengine/volc-sdk-golang/blob/main/example/tls/demo_producer.go
```

## 工作原理

将 GitHub 页面 URL（如 `github.com/.../blob/...`）转换为原始文件 URL（如 `raw.githubusercontent.com/...`），方便直接下载。
