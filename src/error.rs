use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("OpenAI API error: {0}")]
    OpenAI(#[from] async_openai::error::OpenAIError),

    #[error("serialization or deserialization failed")]
    Serde(#[from] serde_json::Error),

    #[error("DotEnv did not load succesfully: {0}")]
    DotEnv(#[from] dotenv::Error),

    #[error("Tracing could not set global default subscriber: {0}")]
    TracingSetGlobalDefault(#[from] tracing::dispatcher::SetGlobalDefaultError),

    #[error("Environment variable not found: {0}")]
    EnvVar(#[from] std::env::VarError),
}
