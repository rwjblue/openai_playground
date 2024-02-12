mod error;

use error::AppError;

use async_openai::{
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
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

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();

    dotenv::dotenv()?;

    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-4-turbo-preview")
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
