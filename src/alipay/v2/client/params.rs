use std::collections::BTreeMap;

use ahash::HashMap;
use cfg_if::cfg_if;
use chrono::FixedOffset;
use serde_json::{json, Value};

use crate::alipay::config::AlipayConfig;

pub(super) type EncodedPayload = Value;
pub(super) type EncodedQuery = String;

pub fn encode(conf: &AlipayConfig, method: &str, payload: &str) -> (EncodedPayload, EncodedQuery) {
    encode_with_custom(conf, method, payload, None)
}

// all leagal param keys
/// ```
/// [
///     "alipay_root_cert_sn",
///     "app_auth_token",
///     "app_cert_sn",
///     "app_id",
///     "auth_token",
///     "charset",
///     "format",
///     "method",
///     "notify_url",
///     "return_url",
///     "sign",
///     "sign_type",
///     "timestamp",
///     "version",
///     "ws_service_url",
/// ];
/// ```
///
pub fn encode_with_custom(
    conf: &AlipayConfig,
    method: &str,
    payload: &str,
    custom: Option<HashMap<String, String>>,
) -> (EncodedPayload, EncodedQuery) {
    let tz = FixedOffset::east_opt(8 * 3600).expect("FixedOffset::east_opt out of bounds");

    let timestamp = chrono::Utc::now()
        .with_timezone(&tz)
        .format("%y-%m-%d %H:%M:%S")
        .to_string();

    let mut params: BTreeMap<String, String> = BTreeMap::from([
        #[cfg(feature = "third_party")]
        ("app_auth_token", conf.keys.app_auth_token.to_owned()),
        ("app_id".to_owned(), conf.app_id.clone()),
        ("charset".to_owned(), "utf-8".to_owned()),
        ("format".to_owned(), "JSON".to_owned()),
        ("method".to_owned(), method.to_owned()),
        ("sign_type".to_owned(), "RSA2".to_owned()),
        ("timestamp".to_owned(), timestamp),
        ("version".to_owned(), "1".to_owned()),
    ]);

    if let Some(custom_params) = custom {
        params.extend(custom_params);
    }

    if let Some(notify_url) = conf.notify_url.clone() {
        params.insert("notify_url".to_owned(), notify_url);
    }

    if let Some(return_url) = conf.return_url.clone() {
        params.insert("return_url".to_owned(), return_url);
    }

    cfg_if! {
        if #[cfg(feature = "alipay_cert")] {
            let pub_key = conf.keys.public.as_ref();

            if let (Some(app_cert_sn), Some(alipay_root_cert_sn)) = (
                pub_key.map(|x| x.app_cert_sn.to_owned()),
                pub_key.map(|x| x.root_cert_sn.to_owned()),
            ) {
                params.insert("app_cert_sn".to_owned(), app_cert_sn.to_owned());
                params.insert(
                    "alipay_root_cert_sn".to_owned(),
                    alipay_root_cert_sn.to_owned(),
                );
            }
        }
    }

    if let Some(ws_service_url) = conf.ws_service_url.as_deref() {
        params.insert("ws_service_url".to_owned(), ws_service_url.to_owned());
    }

    cfg_if! {
        if #[cfg(feature="alipay_aes")] {
            params.insert("encrypt_type".to_owned(), "AES".to_owned());
            let biz_content = conf.keys.aes.encode(biz_content.as_bytes());
        } else {
            let biz_content = payload.to_string();
        }
    }

    params.insert("biz_content".to_owned(), biz_content.clone());

    let (signature, querystring) =
        params
            .iter()
            .fold((String::new(), String::new()), |(sig, query), (k, v)| {
                let sep = if sig.is_empty() { "" } else { "&" };
                let sig = format!("{sig}{sep}{k}={}", urlencoding::encode(v));

                if k != "encrypt_type" && k != "biz_content" {
                    (sig, format!("{query}{sep}{k}={}", urlencoding::encode(v)))
                } else {
                    (sig, query)
                }
            });

    let sign = crate::crypto::with_rsa::encode(conf.keys.private.clone(), signature.as_bytes());

    (
        json!({"biz_content": biz_content}),
        format!("{querystring}&sign={sign}"),
    )
}
