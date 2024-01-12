use base64::{engine::general_purpose::STANDARD, Engine};
use rsa::{pkcs8::DecodePublicKey, sha2::Sha256, RsaPublicKey};
use x509_parser::nom::AsBytes;

use crate::{errors::Kind, Error, Result};

pub fn nonce() -> String {
    uuid::Uuid::new_v4().as_simple().to_string()
}

pub fn encode_signature(priv_key: rsa::RsaPrivateKey, signature: &[u8]) -> String {
    use rsa::pkcs1v15::SigningKey;
    use rsa::signature::RandomizedSigner;

    let mut rng = rand::thread_rng();
    let encoded = SigningKey::<Sha256>::new(priv_key)
        .sign_with_rng(&mut rng, signature)
        .to_string();

    STANDARD.encode(encoded)
}

pub(crate) fn verify_sign(
    pub_key: &[u8],
    msg: &[u8],
    signature: &[u8],
    base64: bool,
) -> Result<()> {
    use rsa::{
        pkcs1v15::{Signature, VerifyingKey},
        signature::Verifier,
    };

    let public_key = RsaPublicKey::from_public_key_der(pub_key)?;

    let signature = if base64 {
        Signature::try_from(STANDARD.decode(signature)?.as_bytes())?
    } else {
        Signature::try_from(signature)?
    };

    let verifying_key: VerifyingKey<Sha256> = VerifyingKey::from(public_key.clone());

    verifying_key.verify(msg, &signature)?;
    Ok(())
}

pub fn encode_sensitive(pub_key: &rsa::RsaPublicKey, msg: &[u8]) -> Result<String> {
    use rsa::Oaep;

    let mut rng = rand::thread_rng();
    let padding = Oaep::new::<Sha256>();

    let input = pub_key
        .encrypt(&mut rng, padding, msg)
        .map_err(|source| Error::new(Kind::InvalidRequest, Some(source)))?;

    Ok(STANDARD.encode(input))
}

pub fn decode_sensitive(priv_key: &rsa::RsaPrivateKey, ciphertext: &str) -> Result<String> {
    use rsa::Oaep;
    let encoded_data = STANDARD
        .decode(ciphertext)
        .map_err(|source| Error::new(Kind::DecodeCiphertext, Some(source)))?;

    let padding = Oaep::new::<Sha256>();
    let decrypted = priv_key
        .decrypt(padding, &encoded_data)
        .map_err(|source| Error::new(Kind::DecodeCiphertext, Some(source)))?;

    Ok(String::from_utf8(decrypted).unwrap())
}
