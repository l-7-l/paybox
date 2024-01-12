use crate::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Certificate {
    pub serial_no: String,
    pub effective_time: String,
    pub expire_time: String,
    pub encrypt_certificate: EncryptCertificate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EncryptCertificate {
    // 【加密证书的算法】 对开启结果数据进行加密的加密算法，目前只支持AEAD_AES_256_GCM。
    pub algorithm: String,
    // 【加密证书的随机串】 对应到加密算法中的IV。
    pub nonce: String,
    // 【加密证书的附加数据】 加密证书的附加数据，固定为“certificate"。
    pub associated_data: String,
    /// 【加密后的证书内容】 使用API KEY和上述参数，可以解密出平台证书的明文。证书明文为PEM格式。（注意：更换证书时会出现PEM格式中的证书失效时间与接口返回的证书弃用时间不一致的情况）
    pub ciphertext: String,
}

#[derive(Debug, Clone)]
pub struct DecryptCertificate {
    /// 序列号
    pub serial_no: String,
    /// 颁发时间
    pub effective_time: String,
    /// 过期时间
    pub expire_time: String,
    /// PublicKey
    pub public_key: String,
    /// 证书
    pub certificate: Vec<u8>,
}

impl DecryptCertificate {
    pub fn from_pem(pem: &[u8]) -> Result<Self> {
        let (_data, x509) = x509_parser::pem::parse_x509_pem(pem)?;
        let cert = x509.parse_x509()?;
        let pk = cert.public_key();

        let sn = cert.serial.to_string();
        let public_key = STANDARD.encode(pk.raw);

        Ok(Self {
            serial_no: sn,
            effective_time: cert.validity.not_before.to_string(),
            expire_time: cert.validity.not_after.to_string(),
            public_key,
            certificate: pem.to_vec(),
        })
    }
}
