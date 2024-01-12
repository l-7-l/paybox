use std::ops::Add;

use crate::{errors::Kind, Error, Result};
use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Method,
};
use serde::Deserialize;

use super::{
    conf::WechatConfig,
    crypto::decode_certificate,
    types::certificate::{Certificate, DecryptCertificate, EncryptCertificate},
};

#[derive(Debug)]
struct Store {
    pub certs: DashMap<String, DecryptCertificate>,
    /** 更新证书时间+12小时后的结果,注意此时间并非平台证书本身的过期时间,而是需要更新的时间 */
    expires_at: DateTime<Utc>,
}

static CERTS: OnceCell<Store> = OnceCell::new();

#[derive(Debug, Deserialize)]
pub struct GetCertificatesResponse {
    pub data: Vec<Certificate>,
}

pub fn get(serial_no: &str) -> Option<DecryptCertificate> {
    CERTS
        .get()
        .and_then(|x| x.certs.get(serial_no).map(|x| x.clone()))
}

pub async fn update_certificates(conf: &WechatConfig, force_update: bool) -> Result<()> {
    let store = CERTS.get();
    // 如果证书过期时间存在,并且证书过期时间大于当前时间,则不更新证书
    if !force_update
        && (store.is_none()
            || store.is_some_and(|x| x.expires_at.timestamp() > Utc::now().timestamp()))
    {
        return Ok(());
    }

    if let Some(certs) = get_certificates(conf).await? {
        CERTS
            .set(Store {
                certs,
                expires_at: Utc::now().add(Duration::hours(12)),
            })
            .map_err(|v| Error::new(Kind::UpdateCertificatesFailed, Some(format!("{:?}", v))))?;
    }

    Ok(())
}

pub async fn get_certificates(
    conf: &WechatConfig,
) -> Result<Option<DashMap<String, DecryptCertificate>>> {
    let url = "https://api.mch.weixin.qq.com/v3/certificates";
    let response = reqwest::Client::new()
        .request(Method::GET, url)
        .header(AUTHORIZATION, conf.signature("GET", url, None)?)
        .header(ACCEPT, "application/json")
        .send()
        .await?;

    if response.status() == 200 {
        let certificates = response.json::<GetCertificatesResponse>().await?.data;

        let certs = DashMap::new();
        let mut decrypt_certs: Vec<DecryptCertificate> = Vec::new();

        for x in certificates.iter() {
            let EncryptCertificate {
                nonce,
                associated_data,
                ciphertext,
                ..
            } = &x.encrypt_certificate;

            let pem = decode_certificate(
                associated_data.as_bytes(),
                nonce.as_bytes(),
                conf.api_key.as_bytes(),
                ciphertext.as_bytes(),
            )?;

            let mut cert = DecryptCertificate::from_pem(&pem)?;
            cert.serial_no = x.serial_no.to_owned();
            cert.effective_time = x.effective_time.to_owned();
            cert.expire_time = x.expire_time.to_owned();

            certs.insert(cert.serial_no.to_owned(), cert.clone());
            decrypt_certs.push(cert);
        }
        Ok(Some(certs))
    } else {
        Ok(None)
    }
}
