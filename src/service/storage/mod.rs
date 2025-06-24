pub mod in_memory_store;

use crate::model::secret::Secret;

pub trait SecretStore: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    fn save(&self, secret: Secret) -> impl Future<Output = Result<Secret, Self::Error>>;
}
