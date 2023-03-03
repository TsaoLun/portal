use graphql_client::Error;
use thiserror::Error;

pub fn get_err(errors: Vec<Error>) -> Result<(), AppError> {
    match errors.get(0) {
        Some(err) => match &err.extensions {
            Some(e) => {
                if let Some(code) = e.get("code") {
                    let c = code.as_str().map(|v| v.to_string());
                    if let Some(c) = c {
                        Err(AppError::SpecError(c))
                    } else {
                        Err(AppError::AnyError(err.message.to_string()))
                    }
                } else {
                    Err(AppError::AnyError(err.message.to_string()))
                }
            }
            None => Err(AppError::AnyError(err.message.to_string())),
        },
        None => Ok(()),
    }
}

pub const SERVER_ERROR: &str = "服务端异常";
pub const PARSER_ERROR: &str = "解析错误";
//pub const AUTH_ERROR: &str = "登录过期，请重新登录";

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Network error occurred: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("{0}")]
    SpecError(String),
    #[error("{0}")]
    AnyError(String),
}
