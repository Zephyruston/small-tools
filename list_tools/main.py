#!/usr/bin/env python3

import json
import os
import subprocess
from datetime import datetime
from pathlib import Path
from typing import List, Dict, Optional
from dotenv import load_dotenv

from openai import OpenAI
from pydantic import BaseModel
from rich.console import Console

# 加载.env文件
load_dotenv()

console = Console()

class ToolInfo(BaseModel):
    name: str
    package_manager: str
    version: Optional[str] = None
    description: Optional[str] = None
    install_command: Optional[str] = None
    path: Optional[str] = None
    install_date: Optional[str] = None

class ToolDiscovery:
    @staticmethod
    def discover_go_tools() -> List[ToolInfo]:
        """发现Go全局安装的包"""
        tools = []
        try:
            # 获取GOPATH
            gopath_result = subprocess.run(
                ['go', 'env', 'GOPATH'], 
                capture_output=True, 
                text=True, 
                check=True
            )
            gopath = gopath_result.stdout.strip()
            
            # 获取GOROOT
            goroot_result = subprocess.run(
                ['go', 'env', 'GOROOT'], 
                capture_output=True, 
                text=True, 
                check=True
            )
            goroot = goroot_result.stdout.strip()
            
            # 检查GOPATH/bin中的可执行文件
            go_bin_path = Path(gopath) / 'bin'
            if go_bin_path.exists():
                for file in go_bin_path.iterdir():
                    if file.is_file() and os.access(file, os.X_OK):
                        tools.append(ToolInfo(
                            name=file.name,
                            package_manager='Go',
                            path=str(file)
                        ))
            
            # 检查GOROOT/bin中的可执行文件(排除标准工具)
            goroot_bin_path = Path(goroot) / 'bin'
            go_bin_tools = {tool.name for tool in tools}  # 已经添加的工具名
            if goroot_bin_path.exists():
                for file in goroot_bin_path.iterdir():
                    if file.is_file() and os.access(file, os.X_OK) and file.name not in go_bin_tools:
                        tools.append(ToolInfo(
                            name=file.name,
                            package_manager='Go',
                            path=str(file)
                        ))
                        
        except subprocess.CalledProcessError:
            console.print("[yellow]警告: 无法获取Go环境信息[/yellow]")
        except Exception as e:
            console.print(f"[yellow]警告: Go工具发现失败: {e}[/yellow]")
            
        return tools

    @staticmethod
    def discover_npm_tools() -> List[ToolInfo]:
        """发现npm全局安装的包"""
        tools = []
        try:
            # 获取全局npm包列表
            result = subprocess.run(
                ['npm', 'list', '-g', '--depth=0', '--json'], 
                capture_output=True, 
                text=True, 
                check=True
            )
            
            data = json.loads(result.stdout)
            if 'dependencies' in data:
                for name, info in data['dependencies'].items():
                    tools.append(ToolInfo(
                        name=name,
                        package_manager='npm',
                        version=info.get('version', ''),
                        description=info.get('description', '')
                    ))
                        
        except subprocess.CalledProcessError:
            console.print("[yellow]警告: 无法获取npm全局包列表[/yellow]")
        except json.JSONDecodeError:
            console.print("[yellow]警告: npm输出解析失败[/yellow]")
        except Exception as e:
            console.print(f"[yellow]警告: npm工具发现失败: {e}[/yellow]")
            
        return tools

    @staticmethod
    def discover_cargo_tools() -> List[ToolInfo]:
        """发现Cargo全局安装的包"""
        tools = []
        try:
            # 获取Cargo主目录 (使用环境变量CARGO_HOME或默认路径)
            cargo_home = os.environ.get('CARGO_HOME')
            if not cargo_home:
                # 如果没有设置CARGO_HOME，使用默认路径 ~/.cargo
                home = os.path.expanduser('~')
                cargo_home = os.path.join(home, '.cargo')
            
            # 检查Cargo安装的二进制文件
            cargo_bin_path = Path(cargo_home) / 'bin'
            if cargo_bin_path.exists():
                for file in cargo_bin_path.iterdir():
                    if file.is_file() and os.access(file, os.X_OK):
                        # 从文件名中移除常见的前缀如 cargo- 等
                        name = file.name
                        if name.startswith('cargo-'):
                            name = name[6:]  # 移除 'cargo-' 前缀
                        
                        tools.append(ToolInfo(
                            name=name,
                            package_manager='Cargo',
                            path=str(file)
                        ))
                        
        except Exception as e:
            console.print(f"[yellow]警告: Cargo工具发现失败: {e}[/yellow]")
            
        return tools

    @staticmethod
    def discover_all_tools() -> List[ToolInfo]:
        """发现所有包管理器的工具"""
        all_tools = []
        
        console.print("[bold blue]正在发现Go工具...[/bold blue]")
        go_tools = ToolDiscovery.discover_go_tools()
        console.print(f"[green]发现 {len(go_tools)} 个Go工具[/green]")
        all_tools.extend(go_tools)
        
        console.print("[bold blue]正在发现npm工具...[/bold blue]")
        npm_tools = ToolDiscovery.discover_npm_tools()
        console.print(f"[green]发现 {len(npm_tools)} 个npm工具[/green]")
        all_tools.extend(npm_tools)
        
        console.print("[bold blue]正在发现Cargo工具...[/bold blue]")
        cargo_tools = ToolDiscovery.discover_cargo_tools()
        console.print(f"[green]发现 {len(cargo_tools)} 个Cargo工具[/green]")
        all_tools.extend(cargo_tools)
        
        return all_tools

class LLMAnalyzer:
    def __init__(self):
        # 只使用OpenAI服务
        self.use_openai = bool(os.getenv("OPENAI_API_KEY"))
        
        if self.use_openai:
            self.openai_client = OpenAI(
                api_key=os.getenv("OPENAI_API_KEY"),
                base_url=os.getenv("OPENAI_BASE_URL", "https://api.openai.com/v1")
            )
            self.openai_model = os.getenv("OPENAI_MODEL", "gpt-3.5-turbo")
        else:
            self.use_openai = False

    def analyze_tools_from_markdown(self, tools_md_path: str = "docs/tools.md") -> str:
        """从保存的Markdown文件中读取工具信息，调用LLM分析生成安装命令"""
        if not self.use_openai:
            console.print("[red]错误: 未配置LLM服务，无法生成安装命令[/red]")
            console.print("[red]请在.env文件中设置OPENAI_API_KEY[/red]")
            return "# 错误: 未配置LLM服务"
        
        # 读取工具文档
        try:
            with open(tools_md_path, 'r', encoding='utf-8') as f:
                tools_data = f.read()
        except FileNotFoundError:
            console.print(f"[red]错误: 未找到工具文档 {tools_md_path}[/red]")
            return "# 错误: 未找到工具文档"
        
        prompt = f"""你是一个专业的开发工具安装专家。请分析以下工具清单，并为每个工具提供准确的安装命令。

要求：
1. 基于工具名称和包管理器类型，推断最可能的安装源
2. 对于Go工具，使用 'go install import-path@version' 格式
3. 对于npm工具，使用 'npm install -g package-name@version' 格式
4. 对于Cargo工具，使用 'cargo install crate-name' 格式
5. 如果是系统内置工具，请标注说明
6. 如果不确定确切路径，请提供最可能的安装命令

工具清单：
{tools_data}

请严格按照JSON格式返回，键为工具名称，值为安装命令：
{{
  "tool1": "install command 1",
  "tool2": "install command 2"
}}

只返回JSON对象，不要包含其他解释文本。"""

        try:
            console.print("[blue]🔄 正在调用LLM API获取安装命令...[/blue]")
            response = self.openai_client.chat.completions.create(
                model=self.openai_model,
                messages=[{"role": "user", "content": prompt}],
                temperature=0.2,
            )
            response_text = response.choices[0].message.content.strip()
            console.print(f"[green]📥 LLM响应长度: {len(response_text)} 字符[/green]")
            return response_text
            
        except Exception as e:
            console.print(f"[red]❌ LLM API调用失败: {e}[/red]")
            return f"# LLM API调用失败: {e}"

    def update_documentation_with_commands(self, tools_md_path: str = "docs/tools.md"):
        """读取markdown文件，调用LLM获取安装命令，并更新文档"""
        if not self.use_openai:
            console.print("[red]错误: 未配置LLM服务，无法生成安装命令[/red]")
            console.print("[red]请在.env文件中设置OPENAI_API_KEY[/red]")
            return
        
        # 读取工具文档
        try:
            with open(tools_md_path, 'r', encoding='utf-8') as f:
                tools_data = f.read()
        except FileNotFoundError:
            console.print(f"[red]错误: 未找到工具文档 {tools_md_path}[/red]")
            return
        
        prompt = f"""你是一个专业的开发工具安装专家。请分析以下工具清单，并为每个工具提供准确的安装命令。

要求：
1. 基于工具名称和包管理器类型，推断最可能的安装源
2. 对于Go工具，使用 'go install import-path@version' 格式
3. 对于npm工具，使用 'npm install -g package-name@version' 格式
4. 对于Cargo工具，使用 'cargo install crate-name' 格式
5. 如果是系统内置工具，请标注说明
6. 如果不确定确切路径，请提供最可能的安装命令

工具清单：
{tools_data}

请提供一个完整的、格式化的Markdown文档，包含原始工具信息和新增的安装命令。
文档应包含以下部分：

1. 标题和生成时间
2. 按包管理器分组的工具列表
3. 每个工具的信息表格（包含工具名称、版本、描述和安装命令四列）

严格按照原始文档的格式，只添加一列安装命令。"""

        try:
            console.print("[blue]🔄 正在调用LLM API生成完整的安装命令文档...[/blue]")
            response = self.openai_client.chat.completions.create(
                model=self.openai_model,
                messages=[{"role": "user", "content": prompt}],
                temperature=0.2,
            )
            updated_content = response.choices[0].message.content.strip()
            
            # 将更新后的内容写入文件
            with open(tools_md_path, 'w', encoding='utf-8') as f:
                f.write(updated_content)
            
            console.print(f"[green]✅ 文档已更新: {tools_md_path}[/green]")
            
        except Exception as e:
            console.print(f"[red]❌ LLM API调用失败: {e}[/red]")

class DocumentationGenerator:
    @staticmethod
    def generate_markdown(tools: List[ToolInfo], output_file: str = "docs/tools.md"):
        """生成Markdown文档"""
        # 确保docs目录存在
        docs_dir = Path("docs")
        docs_dir.mkdir(exist_ok=True)
        
        # 按包管理器分组工具
        grouped_tools: Dict[str, List[ToolInfo]] = {}
        for tool in tools:
            if tool.package_manager not in grouped_tools:
                grouped_tools[tool.package_manager] = []
            grouped_tools[tool.package_manager].append(tool)
        
        # 生成Markdown内容
        content = [
            "# 工具清单",
            "",
            f"生成时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}",
            "",
            "本文档列出了系统中发现的所有开发工具。",
            ""
        ]
        
        # 为每个包管理器生成部分
        for pm, pm_tools in grouped_tools.items():
            content.extend([
                f"## {pm} 工具",
                "",
                f"共发现 {len(pm_tools)} 个 {pm} 工具:",
                ""
            ])
            
            # 添加工具表格
            content.extend([
                "| 工具名称 | 版本 | 描述 |",
                "|---------|------|------|"
            ])
            
            for tool in pm_tools:
                version = tool.version or "未知"
                description = tool.description or "无描述"
                content.append(f"| {tool.name} | {version} | {description} |")
            
            content.append("")
        
        # 写入文件
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write('\n'.join(content))
        
        console.print(f"[green]文档已生成: {output_file}[/green]")

def main():
    """主执行函数 - 发现工具并生成文档"""
    console.print("[bold green]开始工具发现和文档生成过程...[/bold green]")
    
    # 发现工具
    tools = ToolDiscovery.discover_all_tools()
    
    if not tools:
        console.print("[red]未发现任何工具[/red]")
        return
    
    console.print(f"[bold green]总共发现 {len(tools)} 个工具[/bold green]")
    
    # 生成初始文档
    console.print("[bold blue]生成工具文档...[/bold blue]")
    DocumentationGenerator.generate_markdown(tools)
    
    # 使用LLM分析生成安装命令并更新文档
    analyzer = LLMAnalyzer()
    console.print("[bold blue]使用LLM生成安装命令并更新文档...[/bold blue]")
    analyzer.update_documentation_with_commands()
    
    console.print("[bold green]工具发现和文档生成完成![/bold green]")

if __name__ == "__main__":
    main()