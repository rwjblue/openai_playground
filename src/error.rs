use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("OpenAI API error: {0}")]
    OpenAIError(#[from] async_openai::error::OpenAIError),

    #[error("serialization or deserialization failed")]
    SerdeError(#[from] serde_json::Error),
}
