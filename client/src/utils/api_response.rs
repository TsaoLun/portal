use graphql_client::Response;
use thiserror::Error;

pub struct ErrData {
    pub message: String,
    pub code: Option<String>,
}

pub struct ResData<T> {
    pub err: Option<ErrData>,
    pub data: Option<T>,
}

pub fn get_err<T>(response: Response<T>) -> Result<(), AppError> {
    let err = response.errors.map(||);
    let err = err.get(0).unwrap();
    let ext_err = err
        .clone()
        .extensions
        .and_then(|e| e.get("code").map(|code| code.as_str().unwrap().to_string()));
    // (err.clone().message, ext_err);
    todo!()
}

pub const SERVER_ERROR: &str = "服务端异常";

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Network error occurred: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("{0}")]
    TokenError(String),
    #[error("{0}")]
    Other(String),
}

pub type AppResult<T> = Result<T, AppError>;