pub struct ApiResponse<T>(pub Option<T>);

pub struct ErrData {
    pub message: String,
    pub code: Option<String>,
}
