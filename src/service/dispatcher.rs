use crate::service::storage::store::SecretStore;

pub struct Dispatcher {
    storage: Box<dyn SecretStore>,
}

impl Dispatcher {
    pub fn new(storage: Box<dyn SecretStore>) -> Dispatcher {
        Dispatcher { storage }
    }
}
