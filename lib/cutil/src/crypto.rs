use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use anyhow::{anyhow, Context, Result};
use crypto_hash::{hex_digest, Algorithm};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

fn key_iv(password: &str) -> Result<([u8; 16], [u8; 16])> {
    let ki = hex_digest(Algorithm::SHA256, password.as_bytes());
    let ki = hex::decode(ki).context("Decoding key failed")?;

    let (mut key, mut iv) = ([0_u8; 16], [0_u8; 16]);
    key[..].copy_from_slice(&ki[..16]);
    iv[..].copy_from_slice(&ki[16..]);

    Ok((key, iv))
}

pub fn encrypt(password: &str, plain_text: &[u8]) -> Result<String> {
    let len = plain_text.len();
    if len > 4096 {
        return Err(anyhow!(
            "input text is too long, the max text len is 4096 bytes."
        ));
    }

    let (key, iv) = key_iv(password)?;
    let mut buf = [0u8; 4096];
    buf[..len].copy_from_slice(plain_text);

    match Aes128CbcEnc::new(&key.into(), &iv.into()).encrypt_padded_mut::<Pkcs7>(&mut buf, len) {
        Ok(encrypt_text) => Ok(hex::encode(encrypt_text)),
        Err(e) => anyhow::bail!(e.to_string()),
    }
}

pub fn decrypt(password: &str, encrypt_text: &str) -> Result<Vec<u8>> {
    let (key, iv) = key_iv(password)?;
    let mut buf = hex::decode(encrypt_text.as_bytes())?.to_vec();

    match Aes128CbcDec::new(&key.into(), &iv.into()).decrypt_padded_mut::<Pkcs7>(&mut buf) {
        Ok(plain_text) => Ok(Vec::from(plain_text)),
        Err(e) => anyhow::bail!(e.to_string()),
    }
}

pub fn hash(text: &str) -> String {
    hex_digest(
        Algorithm::MD5,
        hex_digest(Algorithm::SHA256, text.as_bytes()).as_bytes(),
    )
}

#[cfg(test)]
mod tests {
    use super::super::str::random_string;
    use super::*;

    #[test]
    fn test_random_string() {
        for i in 1..100 {
            assert_eq!(random_string(i).len(), i);
        }
    }

    #[test]
    fn test_hash() {
        for i in 1..100 {
            let rs = random_string(i);
            let (h1, h2) = (hash(&rs), hash(&rs));
            assert_eq!(h1.len(), 32);
            assert_eq!(h1, h2);
        }
    }

    #[test]
    fn test_encrypt_decrypt() -> Result<()> {
        for i in 1..100 {
            let (text, password) = (random_string(i + 10), random_string(i));
            let enc_text = encrypt(&password, &text.as_bytes())?;
            let dec_text = decrypt(&password, &enc_text)?;
            assert_eq!(text.as_bytes(), dec_text)
        }

        Ok(())
    }
}
