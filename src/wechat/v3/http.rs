use reqwest::{
    header::{ACCEPT, ACCEPT_ENCODING, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{errors::Kind, wechat::conf::WechatConfig, Error};

use super::{certs::update_certificates, pay::error_response::WechatResult};

/// 注意: 查询参数的 ? 需要调用者自行添加
// pub async fn request<Body: Serialize>(
//     conf: &WechatConfig,
//     method: Method,
//     path: &str,
//     body: Option<&Body>,
//     query: Option<&str>,
// ) -> crate::Result<Response> {
//     let url = conf.url(path);
//     let body_str = body.map(serde_json::to_string).transpose()?;

//     let signatrue = conf.signature(method.as_str(), &url, body_str, query)?;

//     let mut builder = reqwest::Client::new().request(method, &url);

//     if query.is_some_and(|v| v.len() > 0) {
//         builder = builder.query(query.unwrap());
//     }

//     if let Some(data) = body {
//         builder = builder.json(data);
//     }

//     let response = builder
//         .header(AUTHORIZATION, signatrue)
//         .header(CONTENT_TYPE, "application/json")
//         .header(ACCEPT, "application/json")
//         .header(ACCEPT_ENCODING, "gzip")
//         .send()
//         .await?;

//     Ok(response)
// }

fn response_handle<T: DeserializeOwned>(res: WechatResult<T>) -> crate::Result<T> {
    match res {
        WechatResult::Ok(x) => Ok(x),
        WechatResult::Err(err) => {
            let response = err.to_string();

            Err(crate::Error::from_kind(Kind::Response {
                code: err.code,
                message: Some(err.message),
                response,
            }))
        }
    }
}
pub async fn upsert<Response: DeserializeOwned, Request: Serialize>(
    conf: &WechatConfig,
    method: Method,
    path: &str,
    body: &Request,
) -> crate::Result<Response> {
    let url = conf.url(path);

    let signatrue = conf.signature(
        method.as_str(),
        &url,
        Some(
            &serde_json::to_string(body)
                .map_err(|source| Error::new(Kind::InvalidRequest, Some(source)))?,
        ),
    )?;

    update_certificates(conf, false).await?;

    let response = reqwest::Client::new()
        .request(method, &url)
        .header(AUTHORIZATION, signatrue)
        .header("Wechatpay-Serial", conf.merchant.serialno())
        .header(ACCEPT, "application/json")
        .json(body)
        .send()
        .await
        .map_err(Error::from)?
        .json::<WechatResult<Response>>()
        .await?;

    response_handle(response)
}

pub async fn get<T: DeserializeOwned>(
    conf: &WechatConfig,
    path: &str,
    query: Option<&str>,
) -> crate::Result<T> {
    let url = conf.url(path);
    let signatrue = conf.signature("GET", &url, query)?;

    update_certificates(conf, false).await?;

    let mut builder = reqwest::Client::new().request(Method::GET, &url);

    if query.is_some_and(|v| !v.is_empty()) {
        builder = builder.query(query.unwrap());
    }

    let response = builder
        .header(AUTHORIZATION, signatrue)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header(ACCEPT_ENCODING, "gzip")
        .send()
        .await?;

    let result = response.json::<T>().await?;

    Ok(result)
}

// pub async fn request<T, D: Serialize + ?Sized, F, Fut>(
//     f: F,
//     conf: &WechatConfig,
//     method: Method,
//     path: &str,
//     data: Option<&D>,
// ) -> crate::Result<T>
// where
//     F: FnOnce(Response) -> Fut,
//     Fut: Future<Output = reqwest::Result<T>> + Send,
// {
//     let url = conf.url(path);
//     let method_str = method.as_str();

//     let signature = conf.signature(
//         "GET",
//         method_str,
//         data.map(serde_json::to_string)
//             .transpose()
//             .map_err(Error::invalid_json)?
//             .as_deref(),
//     )?;

//     let mut builder = reqwest::Client::new().request(Method::GET, &url);

//     if let Some(v) = data {
//         builder = match method {
//             Method::GET => builder.query(v),
//             Method::POST | Method::PUT => builder.json(v),
//             _ => builder,
//         }
//     }

//     let response = builder
//         .header(AUTHORIZATION, signature)
//         .header(CONTENT_TYPE, "application/json")
//         .header(ACCEPT, "application/json")
//         .header(ACCEPT_ENCODING, "gzip")
//         .send()
//         .await
//         .map_err(Error::from)?;

//     // if response.status().is_success() {
//     //     f(response).await.map_err(crate::Error::from)
//     // } else {
//     //     /// TODO: 微信支付响应错误的处理
//     // }
//     f(response).await.map_err(Error::from)
// }
