use crate::domain::vault::errors::CryptoError;
use crate::domain::vault::ports::CryptoEngine;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use rand::RngCore;
const NONCE_LEN: usize = 12;
pub struct ChaCha20Engine;
impl CryptoEngine for ChaCha20Engine {
    fn encrypt_bytes(&self, plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let cipher = build_cipher(key)?;
        let nonce = random_nonce();
        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
        let mut out = nonce.to_vec();
        out.extend(ciphertext);
        Ok(out)
    }
    fn decrypt_bytes(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if data.len() < NONCE_LEN {
            return Err(CryptoError::DecryptionFailed("data too short".into()));
        }
        let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
        let cipher = build_cipher(key)?;
        let nonce = Nonce::from_slice(nonce_bytes);
        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
    }
    fn encrypt_to_file(
        &self,
        plaintext: &[u8],
        dst_path: &str,
        key: &[u8],
    ) -> Result<(), CryptoError> {
        let encrypted = self.encrypt_bytes(plaintext, key)?;
        std::fs::write(dst_path, encrypted).map_err(|e| CryptoError::Io(e.to_string()))
    }
    fn decrypt_from_file(&self, src_path: &str, key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let data = std::fs::read(src_path).map_err(|e| CryptoError::Io(e.to_string()))?;
        self.decrypt_bytes(&data, key)
    }
}
fn build_cipher(key: &[u8]) -> Result<ChaCha20Poly1305, CryptoError> {
    if key.len() != 32 {
        return Err(CryptoError::InvalidKey("key must be exactly 32 bytes".into()));
    }
    Ok(ChaCha20Poly1305::new(Key::from_slice(key)))
}
fn random_nonce() -> Nonce {
    let mut bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut bytes);
    *Nonce::from_slice(&bytes)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::vault::ports::CryptoEngine;
    fn test_key() -> Vec<u8> {
        vec![0x42u8; 32]
    }
    #[test]
    fn encrypt_then_decrypt_round_trips() {
        let engine = ChaCha20Engine;
        let plaintext = b"hello baby vault";
        let key = test_key();
        let ciphertext = engine.encrypt_bytes(plaintext, &key).unwrap();
        let recovered = engine.decrypt_bytes(&ciphertext, &key).unwrap();
        assert_eq!(recovered, plaintext);
    }
    #[test]
    fn ciphertext_is_not_equal_to_plaintext() {
        let engine = ChaCha20Engine;
        let plaintext = b"sensitive data";
        let key = test_key();
        let ciphertext = engine.encrypt_bytes(plaintext, &key).unwrap();
        assert_ne!(&ciphertext[NONCE_LEN..], plaintext.as_slice());
    }
    #[test]
    fn decrypt_with_wrong_key_fails() {
        let engine = ChaCha20Engine;
        let plaintext = b"secret";
        let key = test_key();
        let wrong_key = vec![0xFFu8; 32];
        let ciphertext = engine.encrypt_bytes(plaintext, &key).unwrap();
        let result = engine.decrypt_bytes(&ciphertext, &wrong_key);
        assert!(matches!(result, Err(CryptoError::DecryptionFailed(_))));
    }
    #[test]
    fn wrong_key_length_is_rejected() {
        let engine = ChaCha20Engine;
        let result = engine.encrypt_bytes(b"data", &[0u8; 16]);
        assert!(matches!(result, Err(CryptoError::InvalidKey(_))));
    }
    #[test]
    fn truncated_ciphertext_is_rejected() {
        let engine = ChaCha20Engine;
        let result = engine.decrypt_bytes(&[0u8; 5], &test_key());
        assert!(matches!(result, Err(CryptoError::DecryptionFailed(_))));
    }
    #[test]
    fn each_encryption_produces_different_ciphertext() {
        let engine = ChaCha20Engine;
        let plaintext = b"same data";
        let key = test_key();
        let ct1 = engine.encrypt_bytes(plaintext, &key).unwrap();
        let ct2 = engine.encrypt_bytes(plaintext, &key).unwrap();
        assert_ne!(ct1, ct2, "random nonces should produce different ciphertexts");
    }
}
