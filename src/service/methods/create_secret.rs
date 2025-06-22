use crate::model::secret::Secret;
use crate::service::{dispatcher::Dispatcher, storage::store::Error as StorageError};

use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to store secret: {0}")]
    Storage(#[from] StorageError),
}

pub trait CreateSecret {
    async fn create_secret(&self, secret: Secret) -> Result<(), Error>;
}

impl CreateSecret for Dispatcher {
    async fn create_secret(&self, secret: Secret) -> Result<(), Error> {
        let secret = self.storage.save(secret).await?;
        let secret_message = secret
            .secret_data
            .iter()
            .map(|byte| char::from(*byte))
            .collect::<String>();

        info!("Sucssfully stored the secret: {}", secret_message);

        return Ok(());
    }
}
