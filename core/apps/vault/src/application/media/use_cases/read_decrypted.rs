use crate::application::media::error::MediaError;
use crate::domain::media::repository::MediaRepository;
use crate::domain::vault::ports::CryptoEngine;
use uuid::Uuid;
pub struct ReadDecryptedMediaUseCase<R, C> {
    repository: R,
    crypto: C,
}
impl<R: MediaRepository, C: CryptoEngine> ReadDecryptedMediaUseCase<R, C> {
    pub fn new(repository: R, crypto: C) -> Self {
        Self { repository, crypto }
    }
    pub fn execute(&self, id: Uuid, master_key: &[u8]) -> Result<Vec<u8>, MediaError> {
        let item = self
            .repository
            .find_by_id(id)?
            .ok_or(MediaError::NotFound)?;
        self.crypto
            .decrypt_from_file(&item.encrypted_path, master_key)
            .map_err(MediaError::from)
    }
}
