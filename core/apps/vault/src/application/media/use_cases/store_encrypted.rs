use crate::application::media::error::MediaError;
use crate::domain::media::entity::{MediaItem, NewMediaItem};
use crate::domain::media::repository::MediaRepository;
use crate::domain::vault::ports::CryptoEngine;
use tracing::info;
use uuid::Uuid;
pub struct StoreEncryptedMediaCommand {
    pub title: String,
    pub plaintext_bytes: Vec<u8>,
    pub storage_dir: String,
    pub master_key: Vec<u8>,
}
pub struct StoreEncryptedMediaUseCase<R, C> {
    repository: R,
    crypto: C,
}
impl<R: MediaRepository, C: CryptoEngine> StoreEncryptedMediaUseCase<R, C> {
    pub fn new(repository: R, crypto: C) -> Self {
        Self { repository, crypto }
    }
    pub fn execute(&self, cmd: StoreEncryptedMediaCommand) -> Result<MediaItem, MediaError> {
        if cmd.title.trim().is_empty() {
            return Err(MediaError::TitleEmpty);
        }
        let id = Uuid::new_v4();
        let encrypted_path = format!("{}/{}.enc", cmd.storage_dir, id);
        self.crypto
            .encrypt_to_file(&cmd.plaintext_bytes, &encrypted_path, &cmd.master_key)
            .map_err(MediaError::from)?;
        let size_bytes = cmd.plaintext_bytes.len() as u64;
        let item = self
            .repository
            .create(NewMediaItem { id, title: cmd.title, encrypted_path, size_bytes })
            .map_err(MediaError::from)?;
        info!(media_id = %item.id, "media stored encrypted");
        Ok(item)
    }
}
