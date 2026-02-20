use clap::Parser;
use confy::ConfyError;
use dotenvy::dotenv;
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

/// 应用程序配置
#[derive(Debug, Default, Serialize, Deserialize)]
struct AppConfig {
    /// ModelScope API 基础URL
    api_base_url: Option<String>,
    /// ModelScope API Key
    api_key: Option<String>,
    /// 模型名称
    model_name: Option<String>,
    /// 默认图像尺寸
    default_size: Option<String>,
}

impl AppConfig {
    fn load() -> Result<Self, ConfyError> {
        // 尝试从配置文件加载
        let config: AppConfig = confy::load("image-gen", "config")?;

        // 如果配置文件中的值为空，尝试从环境变量读取作为后备
        let mut cfg = config;

        if cfg.api_base_url.is_none() {
            cfg.api_base_url = env::var("API_BASE_URL").ok();
        }
        if cfg.api_key.is_none() {
            cfg.api_key = env::var("API_KEY").ok();
        }
        if cfg.model_name.is_none() {
            cfg.model_name = env::var("MODEL_NAME").ok();
        }

        // 保存更新后的配置
        confy::store("image-gen", "config", &cfg)?;

        Ok(cfg)
    }
}

/// ModelScope图像生成CLI工具
/// 通过API生成图像，图像以hash值命名保存
#[derive(Parser)]
#[clap(
    name = "image_gen",
    about = "ModelScope图像生成CLI工具",
    version = "0.1.0",
    after_help = "支持的图像尺寸:\n  1664x928 (16:9 横屏, 默认)\n  1472x1140 (4:3 标准横屏)\n  1328x1328 (1:1 正方形)\n  1140x1472 (3:4 标准竖屏)\n  928x1664 (9:16 竖屏)"
)]
struct Cli {
    /// 图像生成提示词
    prompt: String,

    /// 图像尺寸
    #[clap(
        long,
        short,
        value_parser = ["1664x928", "1472x1140", "1328x1328", "1140x1472", "928x1664"],
        default_value = "1664x928"
    )]
    size: String,

    /// 输出文件名
    #[clap(long, short)]
    output: Option<String>,
}

// 支持的图像尺寸 (根据ModelScope API文档，Qwen-Image支持范围[64x64,1664x1664])
fn get_supported_sizes() -> HashMap<&'static str, (u32, u32)> {
    let mut sizes = HashMap::new();
    sizes.insert("1664x928", (1664, 928)); // 16:9 (横屏)
    sizes.insert("1472x1140", (1472, 1140)); // 4:3 (标准横屏)
    sizes.insert("1328x1328", (1328, 1328)); // 1:1 (正方形，默认)
    sizes.insert("1140x1472", (1140, 1472)); // 3:4 (标准竖屏)
    sizes.insert("928x1664", (928, 1664)); // 9:16 (竖屏)
    sizes
}

#[derive(Serialize)]
struct GenerationRequest {
    model: String,
    prompt: String,
    size: String,
}

#[derive(Deserialize)]
struct GenerationResponse {
    task_id: String,
}

#[derive(Deserialize)]
struct TaskResponse {
    task_status: String,
    #[serde(default)]
    output_images: Vec<String>,
}

async fn generate_via_api(
    client: &reqwest::Client,
    prompt: &str,
    size: &str,
    api_base_url: &str,
    api_key: &str,
    model_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("正在生成图像，提示词: {}", prompt);
    println!("图像尺寸: {}", size);

    let supported_sizes = get_supported_sizes();
    let size_key = if supported_sizes.contains_key(size) {
        size
    } else {
        println!("不支持的图像尺寸: {}，使用默认值 1328x1328", size);
        "1328x1328"
    };

    // 构建请求头
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-ModelScope-Async-Mode", "true".parse().unwrap());

    // 构建请求体
    let payload = GenerationRequest {
        model: model_name.to_string(),
        prompt: prompt.to_string(),
        size: size_key.to_string(),
    };

    // 发送请求
    let response = client
        .post(format!("{}v1/images/generations", api_base_url))
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("请求失败: {}", response.status()).into());
    }

    let generation_response: GenerationResponse = response.json().await?;
    let task_id = generation_response.task_id;
    println!("任务已提交，任务ID: {}", task_id);

    // 轮询获取结果
    loop {
        let mut task_headers = reqwest::header::HeaderMap::new();
        task_headers.insert(
            "Authorization",
            format!("Bearer {}", api_key).parse().unwrap(),
        );
        task_headers.insert(
            "X-ModelScope-Task-Type",
            "image_generation".parse().unwrap(),
        );

        let result = client
            .get(format!("{}v1/tasks/{}", api_base_url, task_id))
            .headers(task_headers)
            .send()
            .await?;

        if !result.status().is_success() {
            return Err(format!("获取任务状态失败: {}", result.status()).into());
        }

        let task_response: TaskResponse = result.json().await?;

        match task_response.task_status.as_str() {
            "SUCCEED" => {
                println!("图像生成成功");
                if let Some(image_url) = task_response.output_images.first() {
                    return Ok(image_url.clone());
                } else {
                    return Err("未找到生成的图像URL".into());
                }
            }
            "FAILED" => {
                return Err("图像生成失败".into());
            }
            _ => {
                println!("图像生成中，请稍候...");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

async fn save_image_from_url(
    client: &reqwest::Client,
    url: &str,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;

    let mut file = File::create(filename)?;
    file.write_all(&bytes)?;

    println!("图像已保存为: {}", filename);

    // 发送桌面通知
    if let Err(e) = Notification::new()
        .summary("图像生成完成")
        .body(&format!("图像已保存为: {}", filename))
        .icon("image-x-generic")
        .show()
    {
        eprintln!("发送通知失败: {}", e);
    }

    Ok(())
}

fn generate_filename(prompt: &str) -> String {
    let digest = md5::compute(prompt);
    format!("{:x}.png", digest)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // 先尝试解析 CLI 参数，让 clap 处理 --help / --version
    let cli = match Cli::try_parse_from(std::env::args()) {
        Ok(cli) => cli,
        Err(e) => {
            // clap 会自动打印帮助/版本信息
            e.exit();
        }
    };

    // 加载配置
    let config = AppConfig::load()?;

    // 从配置中获取必需的API参数
    let api_base_url = config
        .api_base_url
        .ok_or("请在配置文件中设置 API_BASE_URL 或设置环境变量")?;
    let api_key = config
        .api_key
        .ok_or("请在配置文件中设置 API_KEY 或设置环境变量")?;
    let model_name = config
        .model_name
        .ok_or("请在配置文件中设置 MODEL_NAME 或设置环境变量")?;

    // 生成文件名
    let output_filename = cli.output.unwrap_or_else(|| generate_filename(&cli.prompt));

    let client = reqwest::Client::new();

    match generate_via_api(
        &client,
        &cli.prompt,
        &cli.size,
        &api_base_url,
        &api_key,
        &model_name,
    )
    .await
    {
        Ok(image_url) => {
            if let Err(e) = save_image_from_url(&client, &image_url, &output_filename).await {
                eprintln!("保存图像时出错: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("错误: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
