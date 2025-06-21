use crate::model::secret::Secret;
use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

pub trait SecretStore {
    fn save(&mut self, secret: Secret) -> Result<Secret, Error>;
}

pub struct InMemorySecretStore {
    pub storage: HashMap<uuid::Uuid, Vec<u8>>,
}

impl SecretStore for InMemorySecretStore {
    fn save(&mut self, secret: Secret) -> Result<Secret, Error> {
        self.storage.insert(secret.id, secret.secret_data.clone());
        Ok(secret)
    }
}

#[cfg(test)]
pub mod tests {}
