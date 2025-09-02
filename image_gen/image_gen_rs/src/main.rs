use clap::Parser;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

/// ModelScope图像生成CLI工具
/// 通过API生成图像，图像以hash值命名保存
#[derive(Parser)]
#[clap(
    name = "ModelScope图像生成工具",
    about = "ModelScope图像生成工具",
    after_help = "支持的图像尺寸:\n  1664x928 (16:9 横屏)\n  1472x1140 (4:3 标准横屏)\n  1328x1328 (1:1 正方形，默认)\n  1140x1472 (3:4 标准竖屏)\n  928x1664 (9:16 竖屏)"
)]
struct Cli {
    /// 图像生成提示词
    prompt: String,

    /// 图像尺寸
    #[clap(
        long,
        short,
        value_parser = ["1664x928", "1472x1140", "1328x1328", "1140x1472", "928x1664"],
        default_value = "1328x1328"
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
    Ok(())
}

fn generate_filename(prompt: &str) -> String {
    let digest = md5::compute(prompt);
    format!("{:x}.png", digest)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // ModelScope API配置
    let api_base_url = env::var("API_BASE_URL")?;
    let api_key = env::var("API_KEY")?;
    let model_name = env::var("MODEL_NAME")?;

    let cli = Cli::parse();

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
