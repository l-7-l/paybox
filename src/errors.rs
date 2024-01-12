use std::{error::Error as StdError, fmt::Display};

pub type Result<T> = std::result::Result<T, Error>;
pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

#[derive(Debug)]
pub struct Error {
    inner: Box<Inner>,
}

impl Error {
    pub(crate) fn new<E>(kind: Kind, source: Option<E>) -> Self
    where
        E: Into<BoxError>,
    {
        Error {
            inner: Box::new(Inner {
                kind,
                source: source.map(Into::into),
            }),
        }
    }

    pub(crate) fn from_kind(kind: Kind) -> Self {
        Error {
            inner: Box::new(Inner { kind, source: None }),
        }
    }

    pub(crate) fn invalid_json(source: serde_json::Error) -> Self {
        Error::new(Kind::InvalidJson, Some(source))
    }

    pub fn kind(&self) -> &Kind {
        &self.inner.kind
    }
    pub fn source(&self) -> Option<&BoxError> {
        self.inner.source.as_ref()
    }
}

#[derive(Debug)]
struct Inner {
    kind: Kind,
    source: Option<BoxError>,
}

#[derive(Debug)]
pub enum Kind {
    InvalidRequest,
    // InvalidResponse,
    InvalidJson,
    HttpClient,
    InvalidSignature,
    EncodeCiphertext,
    DecodeCiphertext,
    UpdateCertificatesFailed,
    Response {
        code: String,
        message: Option<String>,
        response: String,
    },
    Unknown,
}

impl Kind {
    pub fn as_str(&self) -> &str {
        match self {
            Kind::HttpClient => "request_failed",
            Kind::InvalidJson => "invalid_json",
            Kind::InvalidRequest => "invalid_request",
            // Kind::InvalidResponse => "invalid_response",
            Kind::UpdateCertificatesFailed => "update_certificates_failed",
            Kind::InvalidSignature => "invalid_signature",
            Kind::Response { message, code, .. } => message.as_deref().unwrap_or(code.as_str()),
            Kind::EncodeCiphertext => "encode_ciphertext",
            Kind::DecodeCiphertext => "decode_ciphertext",
            Kind::Unknown => "unknown",
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "kind: {}, source: {:?}",
            self.kind().as_str(),
            self.source().map(|x| x.to_string())
        )
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::invalid_json(err)
    }
}

impl From<aes_gcm_siv::Error> for Error {
    fn from(err: aes_gcm_siv::Error) -> Self {
        Error::new(Kind::DecodeCiphertext, Some(err.to_string()))
    }
}

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Error::new(Kind::HttpClient, Some(source))
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Error::new(Kind::InvalidSignature, Some(err))
    }
}

impl From<rsa::signature::Error> for Error {
    fn from(err: rsa::signature::Error) -> Self {
        Error::new(Kind::InvalidRequest, Some(err))
    }
}

impl From<rsa::pkcs8::Error> for Error {
    fn from(err: rsa::pkcs8::Error) -> Self {
        Error::new(Kind::InvalidRequest, Some(err))
    }
}

impl From<rsa::pkcs8::spki::Error> for Error {
    fn from(value: rsa::pkcs8::spki::Error) -> Self {
        Error::new(Kind::InvalidRequest, Some(value))
    }
}

#[cfg(feature = "wechat_pay_v3")]
use crate::wechat::pay::error_response::WechatError;

#[cfg(feature = "wechat_pay_v3")]
impl From<WechatError> for Error {
    fn from(err: WechatError) -> Self {
        let response = err.to_string();

        Error::from_kind(Kind::Response {
            code: err.code,
            message: Some(err.message),
            response,
        })
    }
}

impl From<x509_parser::nom::Err<x509_parser::prelude::PEMError>> for Error {
    fn from(err: x509_parser::nom::Err<x509_parser::prelude::PEMError>) -> Self {
        Error::new(Kind::InvalidRequest, Some(err))
    }
}

impl From<x509_parser::nom::Err<x509_parser::prelude::X509Error>> for Error {
    fn from(err: x509_parser::nom::Err<x509_parser::prelude::X509Error>) -> Self {
        Error::new(Kind::InvalidRequest, Some(err))
    }
}

// #[derive(Debug)]
// #[non_exhaustive]
// pub enum Error {
//     Serde(serde_json::Error),
//     #[cfg(feature = "alipay")]
//     AliResponse(crate::alipay::client::ErrorResponse),
//     #[cfg(feature = "alipay")]
//     // AliUnknownRes {
//     //     trace_id: Option<String>,
//     //     response: String,
//     // },
//     InvalidSignature(String),
//     Rsa(rsa::Error),
//     Reqwest(reqwest::Error),
// }

// impl From<serde_json::Error> for Error {
//     fn from(err: serde_json::Error) -> Self {
//         Self::Serde(err)
//     }
// }

// impl From<base64::DecodeError> for Error {
//     fn from(e: base64::DecodeError) -> Self {
//         Self::InvalidSignature(e.to_string())
//     }
// }

// impl From<rsa::pkcs1::Error> for Error {
//     fn from(e: rsa::pkcs1::Error) -> Self {
//         Self::InvalidSignature(e.to_string())
//     }
// }

// impl From<rsa::Error> for Error {
//     fn from(err: rsa::Error) -> Self {
//         Error::Rsa(err)
//     }
// }

// // impl From<rsa::pkcs8::Error> for Error {
// //     fn from(value: rsa::pkcs8::Error) -> Self {
// //         Error::InvalidSignature(value.to_string())
// //     }
// // }

// // impl From<rsa::pkcs8::spki::Error> for Error {
// //     fn from(value: rsa::pkcs8::spki::Error) -> Self {
// //         Error::InvalidSignature(value.to_string())
// //     }
// // }

// impl From<rsa::signature::Error> for Error {
//     fn from(e: rsa::signature::Error) -> Self {
//         Error::InvalidSignature(e.to_string())
//     }
// }

// impl From<reqwest::Error> for Error {
//     fn from(e: reqwest::Error) -> Self {
//         Self::Reqwest(e)
//     }
// }

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let (module, e) = match self {
//             Error::Reqwest(e) => ("reqwest", e.to_string()),
//             Error::InvalidSignature(e) => ("invalid_signature", e.to_owned()),

//             #[cfg(feature = "alipay")]
//             Error::AliResponse(e) => ("alipay_response_error", e.base.msg.to_owned()),
//             #[cfg(feature = "alipay")]
//             Error::AliUnknownRes { trace_id, response } => (
//                 "alipay_unknown_response",
//                 format!(
//                     "trade_id: {}, response:{response}",
//                     trace_id.as_deref().unwrap_or_default()
//                 ),
//             ),
//             Error::Serde(e) => ("serde", e.to_string()),
//             Error::Rsa(e) => ("rsa", e.to_string()),
//             // Error::Io(e) => ("IO", e.to_string()),
//             // Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
//         };
//         write!(f, "error in {}: {}", module, e)
//     }
// }
