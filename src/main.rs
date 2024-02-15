mod error;

use error::AppError;

use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};

macro_rules! system {
    ($msg:expr) => {{
        // Attempt to create the object, handling potential errors with `?`
        ChatCompletionRequestSystemMessageArgs::default()
            .content($msg)
            .build()?
            .into()
    }};
}

macro_rules! user {
    ($msg:expr) => {{
        // Attempt to create the object, handling potential errors with `?`
        ChatCompletionRequestUserMessageArgs::default()
            .content($msg)
            .build()?
            .into()
    }};
}

macro_rules! assistant {
    ($msg:expr) => {{
        // Attempt to create the object, handling potential errors with `?`
        ChatCompletionRequestAssistantMessageArgs::default()
            .content($msg)
            .build()?
            .into()
    }};
}

type AppResult<Success = ()> = Result<Success, AppError>;

async fn hello_word() -> AppResult {
    tracing_subscriber::fmt::init();

    dotenv::dotenv()?;

    let config = OpenAIConfig::new().with_api_base("http://localhost:11434/v1");
    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("mistral")
        .messages([
            system!("You are a helpful assistant."),
            user!("Who won the world series in 2020?"),
            assistant!("The Los Angeles Dodgers won the World Series in 2020."),
            user!("Where was it played?"),
        ])
        .build()?;

    tracing::info!("Sending request: {}", serde_json::to_string(&request)?);

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }

    Ok(())
}

async fn hello_functions() -> AppResult {
    let config = OpenAIConfig::new().with_api_base("http://localhost:11434/v1");
    let client = Client::with_config(config);

    // TODO: parse json -> tell model bad job if json invalid
    // TODO: take code, run it in python
    //  -> report errors
    //  -> report bad results?  e.g. no errors but calls functions incorrectly
    //
    // TODO: add another function (code_search); update sys prompt + test prompts for using both
    // code search and dependency backend.
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("mistral")
        .messages([
            system!(include_str!("./hello_functions_system_prompt.txt")),
            // user!("Who depends on restli vesion v1.3.6?"),
            user!("I'm looking for users of moo-bar version 2.1"),
        ])
        .build()?;

    tracing::info!("Sending request: {}", serde_json::to_string(&request)?);

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() -> AppResult {
    tracing_subscriber::fmt::init();

    dotenv::dotenv()?;

    // hello_word().await
    hello_functions().await
}
