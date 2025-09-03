use clap::Parser;
use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;
use std::fs;

/// A simple CLI tool for translating Chinese to English.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The Chinese text to translate
    text: String,

    /// Use AI translation service
    #[arg(long)]
    ai: bool,
}

// AI translation service implementation
struct AITranslationService {
    api_key: String,
    base_url: String,
    model: String,
}

impl AITranslationService {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Get API key from environment
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY environment variable not set")?;

        // Get base URL from environment or use default
        let base_url = env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1/".to_string());

        // Get model from environment or use default
        let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());

        Ok(Self {
            api_key,
            base_url,
            model,
        })
    }
}

#[derive(serde::Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(serde::Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(serde::Deserialize)]
struct ChatChoice {
    message: ChatResponseMessage,
}

#[derive(serde::Deserialize)]
struct ChatResponseMessage {
    content: String,
}

#[derive(serde::Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

impl AITranslationService {
    async fn translate(&self, text: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Create the request
        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "You are a professional translator. Translate the following Chinese text to English.".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: text.to_string(),
                },
            ],
        };

        // Make the HTTP request
        let client = reqwest::Client::new();
        let url = format!("{}chat/completions", self.base_url);

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(
                format!("API request failed with status {}: {}", status, error_text).into(),
            );
        }

        let response_body: ChatResponse = response.json().await?;
        let translation = response_body
            .choices
            .first()
            .map(|choice| &choice.message.content)
            .ok_or("No translation received from API")?;

        Ok(translation.clone())
    }
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    let args = Args::parse();

    // Load dictionary
    let dictionary = match load_dictionary() {
        Ok(dict) => dict,
        Err(e) => {
            eprintln!("Error loading dictionary: {}", e);
            return;
        }
    };

    // Perform translation
    match translate(&dictionary, &args.text, args.ai).await {
        Ok(translation) => println!("Translation: {}", translation),
        Err(e) => eprintln!("Error: {}", e),
    }
}

async fn translate(
    dictionary: &HashMap<String, String>,
    text: &str,
    use_ai: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    // Lookup translation in local dictionary
    // The dictionary is structured as English:Chinese, so we need to search the values for the Chinese text
    if let Some((english, _)) = dictionary
        .iter()
        .find(|(_, chinese)| chinese.contains(text))
    {
        return Ok(english.clone());
    }

    // If not found and AI flag is set, use AI service
    if use_ai {
        println!("Using AI translation service");
        let ai_service = AITranslationService::new()?;
        return ai_service.translate(text).await;
    }

    // If not found and AI flag is not set, return an error
    Err(format!(
        "Translation not found for '{}'. Try using --ai flag to get AI translation.",
        text
    )
    .into())
}

fn load_dictionary() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut combined_dict = HashMap::new();

    // Get the dictionary path from environment or use default
    let dict_path = env::var("TRANSLATE_DICT_PATH").unwrap_or_else(|_| {
        dirs::home_dir()
            .map(|home| home.join(".translate/dict"))
            .and_then(|path| path.to_str().map(|s| s.to_string()))
            .ok_or("Could not determine dictionary path")
            .expect("Failed to determine dictionary path")
    });

    // Read all dictionary files in the dictionary directory
    for entry in fs::read_dir(&dict_path)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file and has a .json extension
        if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
            let data = fs::read_to_string(&path)?;
            let dict: HashMap<String, String> = serde_json::from_str(&data)?;

            // Merge the dictionary into the combined dictionary
            for (key, value) in dict {
                combined_dict.insert(key, value);
            }
        }
    }

    Ok(combined_dict)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_dictionary() {
        let dictionary = load_dictionary();
        assert!(dictionary.is_ok());

        let dict = dictionary.unwrap();
        assert!(dict.contains_key("clock-maker"));
        assert_eq!(
            dict.get("clock-maker"),
            Some(&"n. 制造或修理钟表者".to_string())
        );
    }

    #[tokio::test]
    async fn test_local_translation() {
        let dictionary = load_dictionary().unwrap();
        let result = translate(&dictionary, "制造或修理钟表者", false).await;
        assert!(result.is_ok());
        // The dictionary contains multiple entries for "制造或修理钟表者", so we just check that we get one of them
        let translation = result.unwrap();
        assert!(translation == "clock-maker" || translation == "clockmaker");
    }

    #[tokio::test]
    async fn test_local_translation_not_found() {
        let dictionary = load_dictionary().unwrap();
        let result = translate(&dictionary, "nonexistentword", false).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Translation not found")
        );
    }

    // Note: This test requires a valid API key in the environment
    // #[tokio::test]
    // async fn test_ai_translation() {
    //     // This test will only pass if you have a valid API key set in your environment
    //     // Uncomment and run this test only when you have a valid API key
    //     dotenv().ok(); // Load environment variables for this test
    //     let dictionary = load_dictionary().unwrap();
    //     let result = translate(&dictionary, "你好", true).await;
    //     assert!(result.is_ok());
    //     // We can't assert the exact translation as it might vary
    //     // but we can check that it's not empty
    //     assert!(!result.unwrap().is_empty());
    // }
}
