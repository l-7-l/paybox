use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WechatError {
    pub code: String,
    pub message: String,
}
impl Display for WechatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"{{"code": "{}",  "message": "{}"}}"#,
            &self.code, &self.message
        )
    }
}

#[derive(Debug, Deserialize)]
pub enum WechatResult<T> {
    Ok(T),
    Err(WechatError),
}
