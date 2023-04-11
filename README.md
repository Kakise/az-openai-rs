# az-openai-rs

This crate is a library to interface with Azure OpenAI in Rust.

# How-to use

You need to set the following env variables:

```
OPENAI_API_KEY: <Your Azure OpenAI API Key>
OPENAI_API_URL: <Your Azure deployment url>
OPENAI_API_VERSION: <Your deployment version>
OPENAI_API_DEPLOYMENT: <Your deployment name>
```

An example of using the Chat api:

```rust
    let mut messages = Vec::new();
    messages.push((
        "system",
        "You're an AI language model designed to help the user",
    ));
    messages.push(("assistant", "Hello, how can I help you?"));
    messages.push(("user", &query.query));

    let response = completions::chat()
        .messages(messages)
        .max_tokens(456)
        .send();

```

And of using the completion api:
```rust
let response = completions::completion().prompt("Your prompt").send().await;
```
