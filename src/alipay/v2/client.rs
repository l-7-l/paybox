use super::config::AlipayConfig;
use crate::{crypto, errors::Kind, Error, Result};
use cfg_if::cfg_if;
use reqwest::{header::ACCEPT, Client, Method};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

mod params;
mod request;
mod response;

pub use request::AliRequest;
pub use response::{ErrorResponse, Response, UnknownResponse};

fn decode_text_and_signature(text: &str, response_key: &str) -> (String, String) {
    // 待签名的字符串
    let mut text = text.trim();
    // 找到 xxx_response 开始的位置
    let start_idx = text.find(response_key).unwrap();
    // 找到最后一个 “"sign"” 字符串的位置（避免）
    let last_sign_idx = text.rfind(r#""sign""#).unwrap();
    // 7 是 "sign": 的长度
    let sign = text[last_sign_idx + 7..].replace('}', "");
    /*
     * 删除 xxx_response 及之前的字符串
     * 假设原始字符串为
     *  {"xxx_response":{"code":"10000"},"sign":"jumSvxTKwn24G5sAIN"}
     * 删除后变为
     *  :{"code":"10000"},"sign":"jumSvxTKwn24G5sAIN"}
     */
    text = &text[start_idx + response_key.len() + 1..];
    /*
     * 删除最后一个 "sign" 及之后的字符串
     * 删除后变为
     *  :{"code":"10000"},
     * {} 之间就是待验签的字符串
     */
    text = &text[..last_sign_idx];
    // 删除第一个 { 之前 与最后一个 } 之后的任何字符
    let left_braces_idx = text.find('{').unwrap();
    let right_braces_idx = text.find('}').unwrap();
    text = &text[left_braces_idx + 1..right_braces_idx];

    (text.to_owned(), sign)
}

/// 生成请求字符串，用于客户端进行调用
pub fn sdk_exec<P: Serialize>(
    conf: &AlipayConfig,
    method: &str,
    params: P,
) -> crate::Result<String> {
    let payload = serde_json::to_string(&params)
        .map_err(|source| Error::new(Kind::InvalidRequest, Some(source)))?;

    let (_, querystring) = params::encode(conf, method, &payload);

    Ok(querystring)
}

pub async fn request<P: Serialize, T: DeserializeOwned>(
    method: &str,
    conf: &AlipayConfig,
    params: P,
    should_verify_signature: bool,
) -> Result<T> {
    let payload = serde_json::to_string(&params)
        .map_err(|source| Error::new(Kind::InvalidRequest, Some(source)))?;

    let (payload, query) = params::encode(conf, method, &payload);

    // let sep = if conf.gateway.contains('?') { '&' } else { '?' };
    // let url = format!("{}{sep}", conf.gateway);

    let res = Client::new()
        .request(Method::POST, &conf.gateway)
        .timeout(Duration::from_millis(conf.timeout))
        .header(ACCEPT, "text/plain;charset=utf-8")
        .query(&query)
        .json(&payload)
        .send()
        .await
        .map_err(|source| Error::new(Kind::HttpClient, Some(source)))?;

    let is_success = res.status().is_success();
    let trace_id = res
        .headers()
        .get("trace_id")
        .map(|x| x.to_str().unwrap_or_default().to_string());

    let response_text = res
        .text()
        .await
        .map_err(|source| Error::new(Kind::HttpClient, Some(source)))?;

    if !is_success {
        return Err(Error::new(
            Kind::Unknown,
            Some(UnknownResponse {
                trace_id,
                response_text,
            }),
        ));
    }

    let response_key = method.replace('.', "_");

    //   {
    //     "xxx_response": {
    //         "code": "10000",
    //         "msg": "Success",
    //         "order_id": "2018012000502000000005130266"
    //     },
    //     "sign": "ERITJKEIJKJHKKKKKKKHJEREEEEEEEEEEE"
    // }

    let parsed_response =
        serde_json::from_str::<ahash::AHashMap<String, serde_json::Value>>(&response_text)
            .map_err(Error::invalid_json)?;

    // 提取 `xxx_response` 对象
    if let Some(data) = parsed_response.get(&response_key).and_then(|x| x.as_str()) {
        cfg_if! {
            if #[cfg(feature="alipay_aes")] {
                let ciphertext = data;
                let data = conf.keys.aes.decode(ciphertext).map_err(|source| Error::new(Kind::DecodeCiphertext, source));
            }
        }

        // rsa2 校验
        if should_verify_signature {
            let (text, sig) = decode_text_and_signature(data, &response_key);
            crypto::with_rsa::verify(conf.keys.public.clone().map(|x| x.key), &text, &sig)?;
        }

        let result = serde_json::from_str::<Response<T>>(data).map_err(Error::invalid_json)?;

        if result.base.code != "1000" {
            return Err(Error::from_kind(Kind::Response {
                code: result.base.sub_code.unwrap_or(result.base.code),
                message: Some(result.base.sub_msg.unwrap_or(result.base.msg)),
                response: response_text,
            }));
        }

        Ok(result.data)
    } else {
        Err(Error::new(
            Kind::Unknown,
            Some(UnknownResponse {
                trace_id,
                response_text,
            }),
        ))
    }
}
