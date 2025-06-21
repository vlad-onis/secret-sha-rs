use crate::model::secret::Secret;
use crate::service::storage::store::{Error as StorageError, SecretStore};

use tracing::info;
// todo: storage should not be passed to the service as a mutable reference
// todo: don't return the storage error here but make sure to return service specific errors
pub fn create_secret<T: SecretStore>(storage: &mut T, secret: Secret) -> Result<(), StorageError> {
    let secret = storage.save(secret)?;
    let secret_message = secret
        .secret_data
        .iter()
        .map(|byte| char::from(*byte))
        .collect::<String>();

    info!("Sucssfully stored the secret: {}", secret_message);

    return Ok(());
}
