use aes_gcm_siv::{aead::generic_array::GenericArray, AeadInPlace, Aes256GcmSiv, KeyInit, Nonce};
use base64::{engine::general_purpose::STANDARD, Engine};

use crate::Result;

pub fn decode_certificate(
    associated_data: &[u8],
    iv: &[u8],
    key: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>> {
    let ciphertext = STANDARD.decode(ciphertext)?;
    let cipherdata_len = ciphertext.len() - 16;

    let cypherdata_bytes = &ciphertext[..cipherdata_len];
    let auth_tag = &ciphertext[cipherdata_len..];

    let key = GenericArray::from_slice(key);
    let nonce = Nonce::from_slice(iv);
    let tag = GenericArray::from_slice(auth_tag);

    let cipher = Aes256GcmSiv::new(key);

    let mut buffer = Vec::from(cypherdata_bytes);

    cipher.decrypt_in_place_detached(nonce, associated_data, &mut buffer, tag)?;

    Ok(buffer)
}
