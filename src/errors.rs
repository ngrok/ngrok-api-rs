use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error making request request")]
    RequestError(#[from] reqwest::Error),
    #[error("error")]
    Ngrok(NgrokError),
    #[error("unknown error returned: {0}")]
    UnknownError(String),
}

#[derive(Debug, Deserialize)]
pub struct NgrokError {
    pub error_code: String,
    pub msg: String,
}
