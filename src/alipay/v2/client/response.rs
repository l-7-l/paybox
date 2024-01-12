use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BaseResponse {
    /** 响应码。10000 表示成功，其余详见 https://opendocs.alipay.com/common/02km9f */
    pub code: String,
    /** 响应讯息。Success 表示成功。 */
    pub msg: String,
    /** 错误代号 */
    pub sub_code: Option<String>,
    /** 错误辅助信息 */
    pub sub_msg: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response<T> {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(flatten)]
    pub data: T,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    #[serde(flatten)]
    pub base: BaseResponse,

    /** trace id */
    pub trace_id: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct UnknownResponse {
    pub trace_id: Option<String>,
    pub response_text: String,
}

use std::error::Error;
use std::fmt;
impl Error for UnknownResponse {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for UnknownResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Unknown alipay response  \ntrace_id: {:?}\nresponse_text: {}",
            self.trace_id, self.response_text
        )
    }
}
