use crate::get_config;

use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CompletionQuery {
    pub prompt: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
}

#[derive(Serialize)]
pub struct ChatQuery {
    pub messages: Vec<ChatMessages>,
    pub max_tokens: u32,
    pub temperature: f32,
    pub n: u32,
    pub stream: bool,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMessages {
    pub role: String,
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMessageResponse {
    pub message: ChatMessages,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub usage: ChatResponseUsage,
    pub choices: Vec<ChatMessageResponse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatResponseUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompletionChoices {
    pub text: String,
    index: u32,
    finish_reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub choices: Vec<CompletionChoices>,
    pub created: u32,
    pub model: String,
    pub object: String,
}

pub async fn build_completion<T>(query: CompletionQuery) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    let config = get_config();
    let api_url: String;
    api_url = format!(
        "{}/openai/deployments/{}/completions?api-version={}",
        config.openai_api_url, config.openai_api_dplt_name, config.openai_api_version
    );

    println!("API URL: {}", api_url);

    let client = reqwest::Client::new();
    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("api-key", config.openai_api_key)
        .body(serde_json::to_string(&query).unwrap())
        .send()
        .await;

    println!("Query: {:?}", serde_json::to_string(&query).unwrap());

    match &response {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status());
            }
        }
        Err(e) => {
            if e.is_status() {
                return Err(e.status().unwrap());
            } else {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }

    // Parse the response body as Json
    let content = response.unwrap().json::<T>().await;

    match content {
        Ok(s) => Ok(s),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn build_chat<T>(query: ChatQuery) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    let config = get_config();
    let api_url: String;

    api_url = format!(
        "{}/openai/deployments/{}/chat/completions?api-version={}",
        config.openai_api_url, config.openai_api_dplt_name, config.openai_api_version
    );
    println!("API URL: {}", api_url);

    let client = reqwest::Client::new();
    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("api-key", config.openai_api_key)
        .body(serde_json::to_string(&query).unwrap())
        .send()
        .await;

    println!("Query: {:?}", serde_json::to_string(&query).unwrap());
    println!("Response: {:?}", response);

    match &response {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status());
            }
        }
        Err(e) => {
            if e.is_status() {
                return Err(e.status().unwrap());
            } else {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }

    // Parse the response body as Json
    let content = response.unwrap().json::<T>().await;

    match content {
        Ok(s) => Ok(s),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
