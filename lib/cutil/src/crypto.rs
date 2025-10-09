//! Cryptographic utilities for encryption, decryption, and hashing.
//!
//! This module provides AES-128-CBC encryption/decryption and hash functions.

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use anyhow::{anyhow, Context, Result};
use crypto_hash::{hex_digest, Algorithm};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

/// Derives AES-128 key and IV from a password using SHA-256.
///
/// This function takes a password string and derives a 16-byte key and 16-byte IV
/// by hashing the password with SHA-256 and splitting the result.
///
/// # Arguments
///
/// * `password` - The password to derive key and IV from
///
/// # Returns
///
/// Returns a tuple of (key, iv) as 16-byte arrays on success.
fn key_iv(password: &str) -> Result<([u8; 16], [u8; 16])> {
    let ki = hex_digest(Algorithm::SHA256, password.as_bytes());
    let ki = hex::decode(ki).context("Decoding key failed")?;

    let (mut key, mut iv) = ([0_u8; 16], [0_u8; 16]);
    key[..].copy_from_slice(&ki[..16]);
    iv[..].copy_from_slice(&ki[16..]);

    Ok((key, iv))
}

/// Encrypts plain text using AES-128-CBC with a password-derived key.
///
/// # Arguments
///
/// * `password` - The password used to derive the encryption key
/// * `plain_text` - The data to encrypt (max 4096 bytes)
///
/// # Returns
///
/// Returns the encrypted data as a hex-encoded string on success.
///
/// # Errors
///
/// Returns an error if:
/// - The input text exceeds 4096 bytes
/// - Key derivation fails
/// - Encryption fails
///
/// # Examples
///
/// ```
/// use cutil::crypto::encrypt;
///
/// let encrypted = encrypt("mypassword", b"hello world").unwrap();
/// assert!(!encrypted.is_empty());
/// ```
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

/// Decrypts AES-128-CBC encrypted text using a password-derived key.
///
/// # Arguments
///
/// * `password` - The password used to derive the decryption key
/// * `encrypt_text` - The hex-encoded encrypted data to decrypt
///
/// # Returns
///
/// Returns the decrypted data as a byte vector on success.
///
/// # Errors
///
/// Returns an error if:
/// - Key derivation fails
/// - Hex decoding fails
/// - Decryption fails
///
/// # Examples
///
/// ```
/// use cutil::crypto::{encrypt, decrypt};
///
/// let encrypted = encrypt("mypassword", b"hello world").unwrap();
/// let decrypted = decrypt("mypassword", &encrypted).unwrap();
/// assert_eq!(decrypted, b"hello world");
/// ```
pub fn decrypt(password: &str, encrypt_text: &str) -> Result<Vec<u8>> {
    let (key, iv) = key_iv(password)?;
    let mut buf = hex::decode(encrypt_text.as_bytes())?.to_vec();

    match Aes128CbcDec::new(&key.into(), &iv.into()).decrypt_padded_mut::<Pkcs7>(&mut buf) {
        Ok(plain_text) => Ok(Vec::from(plain_text)),
        Err(e) => anyhow::bail!(e.to_string()),
    }
}

/// Computes a hash of the input text using SHA-256 followed by MD5.
///
/// This function first hashes the input with SHA-256, then hashes the result with MD5,
/// producing a 32-character hex string.
///
/// # Arguments
///
/// * `text` - The text to hash
///
/// # Returns
///
/// Returns the hash as a 32-character hex string.
///
/// # Examples
///
/// ```
/// use cutil::crypto::hash;
///
/// let hash1 = hash("hello world");
/// let hash2 = hash("hello world");
/// assert_eq!(hash1, hash2);
/// assert_eq!(hash1.len(), 32);
/// ```
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
