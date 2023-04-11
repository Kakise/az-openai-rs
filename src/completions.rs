use crate::query_builder::{
    ChatMessages, ChatQuery, ChatResponse, CompletionQuery, CompletionResponse,
};
use reqwest::StatusCode;

pub struct Where<'a> {
    query: Vec<(&'a str, String)>,
}

pub struct WhereChat<'a> {
    query: Vec<(&'a str, String)>,
    messages: Vec<(&'a str, String)>,
}

pub fn chat<'a>() -> WhereChat<'a> {
    WhereChat {
        query: Vec::new(),
        messages: Vec::new(),
    }
}

pub fn completions<'a>() -> Where<'a> {
    Where { query: Vec::new() }
}

impl<'a> Where<'a> {
    pub fn prompt(mut self, input: &'a str) -> Self {
        self.query.push(("prompt", String::from(input)));
        self
    }

    pub async fn send(self) -> Result<CompletionResponse, StatusCode> {
        let mut query: CompletionQuery = CompletionQuery {
            prompt: String::new(),
            max_tokens: 16,
            temperature: 1.0,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
        };

        for (k, v) in self.query.into_iter() {
            if k == "prompt" {
                query.prompt = v;
            }
        }

        let result: Result<CompletionResponse, StatusCode> =
            crate::query_builder::build_completion(query).await;

        match result {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        }
    }
}

impl<'a> WhereChat<'a> {
    pub fn messages(mut self, messages: Vec<(&'a str, &'a str)>) -> Self {
        for (role, content) in messages.into_iter() {
            self.messages.push((role, String::from(content)));
        }
        self
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.query.push(("max_tokens", max_tokens.to_string()));
        self
    }

    pub async fn send(self) -> Result<ChatResponse, StatusCode> {
        let mut query: ChatQuery = ChatQuery {
            messages: Vec::new(),
            max_tokens: 16,
            temperature: 1.0,
            n: 1,
            stream: false,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
        };

        for (k, v) in self.query.into_iter() {
            if k == "max_tokens" {
                query.max_tokens = v.parse::<u32>().unwrap();
            }
        }

        for (role, content) in self.messages.into_iter() {
            let new_msg: ChatMessages = ChatMessages {
                role: String::from(role),
                content: String::from(content),
            };
            query.messages.push(new_msg);
        }

        let result: Result<ChatResponse, StatusCode> =
            crate::query_builder::build_chat(query).await;

        match result {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        }
    }
}
