"""
ModelScope图像生成CLI工具
通过API生成图像，图像以hash值命名保存
"""

import argparse
import hashlib
import json
import time
import requests
import os
from PIL import Image
from io import BytesIO
from dotenv import load_dotenv

# 加载.env文件中的环境变量
load_dotenv()

# ModelScope API配置
API_BASE_URL = os.getenv("API_BASE_URL")
API_KEY = os.getenv("API_KEY")
MODEL_NAME = os.getenv("MODEL_NAME")

# 支持的图像尺寸 (根据ModelScope API文档，Qwen-Image支持范围[64x64,1664x1664])
SUPPORTED_SIZES = {
    "1664x928": (1664, 928),  # 16:9 (横屏)
    "1472x1140": (1472, 1140),  # 4:3 (标准横屏)
    "1328x1328": (1328, 1328),  # 1:1 (正方形，默认)
    "1140x1472": (1140, 1472),  # 3:4 (标准竖屏)
    "928x1664": (928, 1664),  # 9:16 (竖屏)
}


def generate_via_api(prompt, size="1328x1328"):
    """通过ModelScope API生成图像"""
    print(f"正在生成图像，提示词: {prompt}")
    print(f"图像尺寸: {size}")

    if size not in SUPPORTED_SIZES:
        print(f"不支持的图像尺寸: {size}，使用默认值 1328x1328")
        size = "1328x1328"

    # 构建请求头
    headers = {
        "Authorization": f"Bearer {API_KEY}",
        "Content-Type": "application/json",
        "X-ModelScope-Async-Mode": "true",
    }

    # 构建请求体，添加size参数 (注意使用"x")
    payload = {
        "model": MODEL_NAME,
        "prompt": prompt,
        "size": size,
    }

    # 发送请求
    response = requests.post(
        f"{API_BASE_URL}v1/images/generations",
        headers=headers,
        data=json.dumps(payload, ensure_ascii=False).encode("utf-8"),
    )

    response.raise_for_status()
    task_id = response.json()["task_id"]
    print(f"任务已提交，任务ID: {task_id}")

    # 轮询获取结果
    while True:
        result = requests.get(
            f"{API_BASE_URL}v1/tasks/{task_id}",
            headers={
                "Authorization": f"Bearer {API_KEY}",
                "X-ModelScope-Task-Type": "image_generation",
            },
        )
        result.raise_for_status()
        data = result.json()

        if data["task_status"] == "SUCCEED":
            print("图像生成成功")
            return data["output_images"][0]
        elif data["task_status"] == "FAILED":
            raise Exception("图像生成失败")

        print("图像生成中，请稍候...")
        time.sleep(5)


def save_image_from_url(url, filename):
    """从URL下载并保存图像"""
    response = requests.get(url)
    response.raise_for_status()
    image = Image.open(BytesIO(response.content))
    image.save(filename)
    print(f"图像已保存为: {filename}")


def generate_filename(prompt):
    """根据提示词生成hash文件名"""
    hash_object = hashlib.md5(prompt.encode())
    return f"{hash_object.hexdigest()}.png"


def main():
    parser = argparse.ArgumentParser(
        description="ModelScope图像生成工具",
        epilog="支持的图像尺寸:\n  1664x928 (16:9 横屏)\n  1472x1140 (4:3 标准横屏)\n  1328x1328 (1:1 正方形，默认)\n  1140x1472 (3:4 标准竖屏)\n  928x1664 (9:16 竖屏)",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("prompt", help="图像生成提示词")
    parser.add_argument(
        "--size",
        choices=["1664x928", "1472x1140", "1328x1328", "1140x1472", "928x1664"],
        default="1328x1328",
        help="图像尺寸 (默认: 1328x1328)",
    )
    parser.add_argument("--output", help="输出文件名 (默认: 根据提示词生成hash文件名)")

    args = parser.parse_args()

    # 生成文件名
    output_filename = args.output if args.output else generate_filename(args.prompt)

    try:
        # 通过API生成
        image_url = generate_via_api(args.prompt, args.size)
        save_image_from_url(image_url, output_filename)
    except Exception as e:
        print(f"错误: {e}")
        return 1

    return 0


if __name__ == "__main__":
    exit(main())
