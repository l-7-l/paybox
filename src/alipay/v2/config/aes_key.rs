use aes::cipher::{KeyIvInit, StreamCipher};
use base64::{engine::general_purpose::STANDARD, DecodeError, Engine};
pub type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "String")]
pub struct AesKey {
    inner: Vec<u8>,
}

impl AesKey {
    pub fn new(key: String) -> Result<Self, DecodeError> {
        Ok(AesKey {
            inner: STANDARD.decode(key)?,
        })
    }
    fn cipher(&self) -> Aes128Ctr64LE {
        Aes128Ctr64LE::new(self.as_bytes().into(), &[0; 16].into())
    }

    pub fn encode(&self, data: &[u8]) -> String {
        let mut cipher = Aes128Ctr64LE::new(self.as_bytes().into(), &[0; 16].into());
        let mut buf = data.to_vec();
        cipher.apply_keystream(&mut buf);
        STANDARD.encode(&buf)
    }

    pub fn decode(&self, ciphertext: &str) -> Result<String, DecodeError> {
        let mut cipher = self.cipher();
        let mut buf = STANDARD.decode(ciphertext)?;

        cipher.apply_keystream(&mut buf);

        Ok(String::from_utf8(buf).expect("Invalid UTF8"))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }
}

impl TryFrom<String> for AesKey {
    type Error = DecodeError;

    fn try_from(key: String) -> Result<Self, Self::Error> {
        AesKey::new(key)
    }
}
