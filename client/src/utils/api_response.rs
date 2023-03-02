use graphql_client::Error;
use thiserror::Error;

pub struct ErrData {
    pub message: String,
    pub code: Option<String>,
}

pub struct ResData<T> {
    pub err: Option<ErrData>,
    pub data: Option<T>,
}

pub fn get_err(errors: &Vec<Error>) -> Result<(), AppError> {
    match errors.get(0) {
        Some(err) => {
            let code = err
                .extensions
                .clone()
                .map(|e| e.get("code"))
                .and_then(|c| c);
            if let Some(&cd) = code {
                let code_str = cd.as_str().unwrap();
                
                return Err(AppError::SpecError(code_str.to_string()));
            } else {
                return Err(AppError::AnyError(err.message.to_string()));
            }
        }
        None => Ok(()),
    }
}

pub const SERVER_ERROR: &str = "服务端异常";
pub const PARSER_ERROR: &str = "解析错误";
pub const AUTH_ERROR: &str = "登录过期，请重新登录";

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Network error occurred: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("{0}")]
    SpecError(String),
    #[error("{0}")]
    AnyError(String),
}

pub type AppResult<T> = Result<T, AppError>;
