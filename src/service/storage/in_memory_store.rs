use crate::model::secret::Secret;
use std::collections::HashMap;

use thiserror::Error;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Debug, Error)]
pub enum Error {}

#[derive(Default, Debug)]
pub struct InMemorySecretStore {
    pub storage: Mutex<HashMap<uuid::Uuid, Vec<u8>>>,
}

impl InMemorySecretStore {
    pub fn new() -> Self {
        InMemorySecretStore {
            storage: Mutex::new(HashMap::new()),
        }
    }

    pub async fn save(&self, secret: Secret) -> Result<Secret, Error> {
        let mut guard = self.storage.lock().await;

        guard.insert(secret.id, secret.secret_data.clone());

        Ok(secret)
    }
}

#[cfg(test)]
pub mod tests {}
