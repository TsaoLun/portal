use graphql_client::Response;

pub struct ErrData {
    pub message: String,
    pub code: Option<String>,
}

pub struct ResData<T> {
    pub err: Option<ErrData>,
    pub data: Option<T>,
}

pub fn get_err<T>(response: Response<T>) -> (String, Option<String>) {
    let err = response.errors.unwrap();
    let err = err.get(0).unwrap();
    let ext_err = err
        .clone()
        .extensions
        .and_then(|e| e.get("code").map(|code| code.as_str().unwrap().to_string()));
    (err.clone().message, ext_err)
}

pub const SERVER_ERROR: &str = "服务端异常";
