use crate::application::vault::error::VaultError;
use zeroize::Zeroizing;
pub struct UnlockCommand {
    pub raw_key: Vec<u8>,
}
pub struct UnlockUseCase;
impl UnlockUseCase {
    pub fn execute(cmd: UnlockCommand) -> Result<Zeroizing<Vec<u8>>, VaultError> {
        if cmd.raw_key.len() != 32 {
            return Err(VaultError::KeyDerivationFailed(
                "master key must be 32 bytes".into(),
            ));
        }
        Ok(Zeroizing::new(cmd.raw_key))
    }
}
