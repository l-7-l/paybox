use self::{crypto::verify_sign, types::signature_header::SignatureHeader};

pub(crate) mod http;

pub mod certs;
pub mod conf;
pub mod crypto;
#[cfg(feature = "platform")]
pub mod merchant;
pub mod pay;
pub mod types;

pub fn verify_notify_sign(header: &SignatureHeader, data: &str) -> bool {
    let Some(cert) = certs::get(&header.serial) else {
        return false;
    };

    let msg = format!("{}\n{}\n{}\n", header.time_stamp, header.nonce, data);

    verify_sign(
        cert.public_key.as_bytes(),
        msg.as_bytes(),
        header.signature.as_bytes(),
        true,
    )
    .is_ok()
}

pub fn verify_signature(pub_key: &[u8], msg: &[u8], signature: &[u8]) -> bool {
    verify_sign(pub_key, msg, signature, false).is_ok()
}
