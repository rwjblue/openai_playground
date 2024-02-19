mod error;

use error::AppError;

use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
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

fn parse_json(json_str: &str) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str::<serde_json::Value>(json_str)
}

async fn execute_query_to_json(
    messages: Vec<ChatCompletionRequestMessage>,
) -> Result<serde_json::Value, AppError> {
    let config = OpenAIConfig::new().with_api_base("http://localhost:11434/v1");
    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("mistral")
        .messages(messages)
        .build()?;

    tracing::info!("Sending request: {}", serde_json::to_string(&request)?);

    let response = client.chat().create(request).await?;

    if let Some(first_choice) = response.choices.first() {
        match &first_choice.message.content {
            Some(possible_json) => match parse_json(possible_json) {
                Ok(json) => {
                    println!("\n\n Found valid JSON: {}", json);
                    Ok(json)
                }
                Err(e) => {
                    println!("Not valid JSON: {:?}", e);
                    Err(AppError::Serde(e))
                }
            },
            None => todo!(),
        }
    } else {
        println!("No choices are available.");
        todo!();
    }
}

async fn hello_functions() -> AppResult {
    // TODO: parse json -> tell model bad job if json invalid
    // TODO: take code, run it in python
    //  -> report errors
    //  -> report bad results?  e.g. no errors but calls functions incorrectly
    //
    // TODO: add another function (code_search); update sys prompt + test prompts for using both
    // code search and dependency backend.

    execute_query_to_json(
        [
            system!(include_str!("./hello_functions_system_prompt.txt")),
            user!("Who depends on restli version v1.3.6?"),
        ]
        .into(),
    )
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> AppResult {
    tracing_subscriber::fmt::init();

    dotenv::dotenv()?;

    // hello_word().await
    hello_functions().await
}
