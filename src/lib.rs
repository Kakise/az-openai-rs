pub mod completions;
mod query_builder;

pub struct Config {
    pub openai_api_key: String,
    pub openai_api_url: String,
    pub openai_api_version: String,
    pub openai_api_dplt_name: String,
}

fn get_config() -> Config {
    Config {
        openai_api_key: std::env::var("OPENAI_API_KEY").unwrap(),
        openai_api_url: std::env::var("OPENAI_API_URL").unwrap(),
        openai_api_version: std::env::var("OPENAI_API_VERSION").unwrap(),
        openai_api_dplt_name: std::env::var("OPENAI_API_DEPLOYMENT").unwrap(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
