use aes::Aes256;
use anyhow::{anyhow, Context, Result};
use block_modes::{block_padding, BlockMode, Cbc};
use crypto_hash::{hex_digest, Algorithm};

type Aes256Cbc = Cbc<Aes256, block_padding::Pkcs7>;

fn key_iv(password: &str) -> Result<(Vec<u8>, Vec<u8>)> {
    let key = hex_digest(Algorithm::SHA256, password.as_bytes());
    let key = hex::decode(key).context("Decoding key failed")?;

    let iv = hex_digest(Algorithm::MD5, password.as_bytes());
    let iv = hex::decode(iv).context("Decoding iv failed")?;
    Ok((key, iv))
}

pub fn encrypt(password: &str, plain_text: &[u8]) -> Result<String> {
    let (key, iv) = key_iv(password)?;
    let cipher = Aes256Cbc::new_from_slices(&key, &iv)?;

    let pos = plain_text.len();
    if pos > 4096 {
        return Err(anyhow!(
            "input text is too long, the max text len is 4096 bytes."
        ));
    }

    let mut buffer = [0u8; 4096];
    buffer[..pos].copy_from_slice(plain_text);
    let text = cipher.encrypt(&mut buffer, pos)?;

    Ok(hex::encode(text))
}

pub fn decrypt(password: &str, encrypt_text: &str) -> Result<Vec<u8>> {
    let (key, iv) = key_iv(password)?;

    let cipher = Aes256Cbc::new_from_slices(&key, &iv)?;
    let mut buf = hex::decode(encrypt_text.as_bytes())?.to_vec();
    let text = cipher.decrypt(&mut buf)?;
    Ok(Vec::from(text))
}

pub fn hash(text: &str) -> String {
    hex_digest(
        Algorithm::MD5,
        hex_digest(Algorithm::SHA256, text.as_bytes()).as_bytes(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::str::random_string;

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
