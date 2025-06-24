use crate::{model::secret::Secret, service::storage::SecretStore};
use std::{collections::HashMap, sync::Arc};

use thiserror::Error;
use tokio::sync::Mutex;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Shit hit the fan")]
    FEK,
}

#[derive(Default, Debug, Clone)]
pub struct InMemorySecretStore {
    pub storage: Arc<Mutex<HashMap<uuid::Uuid, Vec<u8>>>>,
}

impl InMemorySecretStore {
    pub fn new() -> Self {
        InMemorySecretStore {
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl SecretStore for InMemorySecretStore {
    type Error = Error;
    async fn save(&self, secret: Secret) -> Result<Secret, Error> {
        let mut guard = self.storage.lock().await;
        guard.insert(secret.id, secret.secret_data.clone());

        Ok(secret)
    }
}
