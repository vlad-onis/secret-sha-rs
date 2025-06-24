use crate::model::secret::Secret;
use crate::service::{dispatcher::Dispatcher, storage::SecretStore};

use thiserror::Error;
use tracing::{error, info};

#[derive(Error, Debug)]
pub enum Error<StoreError: std::error::Error + Send + Sync + 'static> {
    #[error("Failed to store secret because: {0}")]
    Storage(#[from] StoreError),
}

pub trait CreateSecret<StoreError: std::error::Error + Send + Sync + 'static> {
    fn create_secret(&self, secret: Secret) -> impl Future<Output = Result<(), Error<StoreError>>>;
}

impl<S> CreateSecret<<S as SecretStore>::Error> for Dispatcher<S>
where
    S: SecretStore,
{
    async fn create_secret(&self, secret: Secret) -> Result<(), Error<<S as SecretStore>::Error>> {
        let secret = self.storage.save(secret).await?;
        // .map_err(|e| {
        // error!("Failed to store secret because: {}", e);
        // Error::Storage})?;
        let secret_message = secret
            .secret_data
            .iter()
            .map(|byte| char::from(*byte))
            .collect::<String>();

        info!("Sucssfully stored the secret: {}", secret_message);

        Ok(())
    }
}
