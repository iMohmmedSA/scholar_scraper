use reqwest::StatusCode;



#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to read arguments")]
    ReadArgs,
    
    #[error("Failed to read file")]
    ReadFile,

    #[error("Failed to access the {0}")]
    FailedFetch(String),

    #[error("HTTP request failed with status code {0}")]
    HttpError(StatusCode),

    #[error("Failed to convert response with id: {0}")]
    ConvertText(String),

    #[error("Failed to convert JSON response")]
    ConvertJson,

    #[error("Failed to read JSON response")]
    ReadJson,

    // Remove this error
    #[error("Failed Processing")]
    FailedProcess,
}