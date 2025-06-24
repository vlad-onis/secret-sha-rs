use std::sync::Arc;

use crate::service::storage::SecretStore;

#[derive(Clone, Debug)]
pub struct Dispatcher<S> {
    pub storage: Arc<S>,
}

impl<S: SecretStore> Dispatcher<S> {
    pub fn new(storage: Arc<S>) -> Self {
        Dispatcher { storage }
    }
}
