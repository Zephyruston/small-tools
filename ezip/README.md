# ezip - 简易压缩/解压工具

一个轻量级的命令行压缩/解压工具，支持多种压缩格式。

## 安装

将 `ezip` 脚本添加到 PATH 环境变量中：

```bash
cp ezip ~/.local/bin/
# 或
sudo mv ezip /usr/local/bin/
```

## 使用方法

### 压缩文件

```bash
ezip c <格式> <压缩包名(不带后缀)> <文件1> [文件2]...
```

支持的格式：
- `gz` - gzip 格式
- `bz2` - bzip2 格式
- `xz` - LZMA 格式
- `tar` - tar 归档

示例：
```bash
ezip c gz my_backup file1 dir2 photo.jpg
```

### 解压文件

```bash
ezip x <文件名>
```

示例：
```bash
ezip x my_backup.tar.gz
```
