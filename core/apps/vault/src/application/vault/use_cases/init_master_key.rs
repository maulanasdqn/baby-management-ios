use crate::application::vault::error::VaultError;
use rand::RngCore;
pub struct InitMasterKeyUseCase;
impl InitMasterKeyUseCase {
    pub fn execute() -> Result<Vec<u8>, VaultError> {
        let mut key = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);
        Ok(key)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generates_32_byte_key() {
        let key = InitMasterKeyUseCase::execute().unwrap();
        assert_eq!(key.len(), 32);
    }
    #[test]
    fn generated_keys_are_not_all_zeros() {
        let key = InitMasterKeyUseCase::execute().unwrap();
        assert!(key.iter().any(|&b| b != 0), "key should not be all zeros");
    }
    #[test]
    fn successive_keys_are_unique() {
        let key1 = InitMasterKeyUseCase::execute().unwrap();
        let key2 = InitMasterKeyUseCase::execute().unwrap();
        assert_ne!(key1, key2, "two generated keys must differ");
    }
}
