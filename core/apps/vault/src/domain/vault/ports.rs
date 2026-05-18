use crate::domain::vault::errors::CryptoError;
pub trait CryptoEngine: Send + Sync {
    fn encrypt_bytes(&self, plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn decrypt_bytes(&self, ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn encrypt_to_file(
        &self,
        plaintext: &[u8],
        dst_path: &str,
        key: &[u8],
    ) -> Result<(), CryptoError>;
    fn decrypt_from_file(&self, src_path: &str, key: &[u8]) -> Result<Vec<u8>, CryptoError>;
}
impl<T: CryptoEngine + ?Sized> CryptoEngine for &T {
    fn encrypt_bytes(&self, plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        (**self).encrypt_bytes(plaintext, key)
    }
    fn decrypt_bytes(&self, ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        (**self).decrypt_bytes(ciphertext, key)
    }
    fn encrypt_to_file(&self, plaintext: &[u8], dst_path: &str, key: &[u8]) -> Result<(), CryptoError> {
        (**self).encrypt_to_file(plaintext, dst_path, key)
    }
    fn decrypt_from_file(&self, src_path: &str, key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        (**self).decrypt_from_file(src_path, key)
    }
}
