use base64::{engine::general_purpose::STANDARD, Engine};
use rsa::{
    pkcs1v15::{Signature, SigningKey, VerifyingKey},
    sha2::Sha256,
    signature::RandomizedSigner,
    RsaPublicKey,
};

use crate::{errors::Kind, Error, Result};

pub fn encode(priv_key: rsa::RsaPrivateKey, signature: &[u8]) -> String {
    let mut rng = rand::thread_rng();
    // let priv_key = RsaPrivateKey::from_pkcs8_pem(private_pem)?;

    STANDARD.encode(
        SigningKey::<Sha256>::new(priv_key)
            .sign_with_rng(&mut rng, signature)
            .to_string(),
    )
}

pub fn verify(pub_key: Option<RsaPublicKey>, text: &str, sig: &str) -> Result<()> {
    if let Some(pub_key) = pub_key {
        use rsa::signature::Verifier;

        let sign = STANDARD
            .decode(sig)
            .map_err(|source| Error::new(Kind::DecodeCiphertext, Some(source)))?;

        let verifying_key = VerifyingKey::<Sha256>::new(pub_key);

        let signature = Signature::try_from(sign.as_slice())
            .map_err(|source| Error::new(Kind::InvalidSignature, Some(source)))?;

        verifying_key
            .verify(text.as_bytes(), &signature)
            .map_err(|source| Error::new(Kind::InvalidSignature, Some(source)))?;
    }

    Ok(())
}
