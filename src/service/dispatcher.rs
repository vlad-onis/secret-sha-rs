use std::sync::Arc;

use crate::service::storage::in_memory_store::InMemorySecretStore;

#[derive(Clone, Debug)]
pub struct Dispatcher {
    pub storage: Arc<InMemorySecretStore>,
}

impl Dispatcher {
    pub fn new(storage: Arc<InMemorySecretStore>) -> Dispatcher {
        Dispatcher { storage }
    }
}
