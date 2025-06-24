use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use tracing::{error, info};

use crate::{
    model,
    service::{dispatcher::Dispatcher, methods::create_secret::CreateSecret, storage::SecretStore},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewSecretParams {
    #[serde(rename(serialize = "secret", deserialize = "secret"))]
    base64_encrypted_secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewSecretResponse {
    /// uuid as bytes
    pub secret_id: [u8; 16],
}

pub async fn handle<S>(
    State(dispatcher): State<Dispatcher<S>>,
    Json(params): Json<NewSecretParams>,
) -> impl IntoResponse
where
    S: SecretStore + Clone + Send + Sync + 'static, // S needs to be Clone+Send+Sync for Axum State
    // Crucially, Dispatcher<S> needs to implement CreateSecret,
    // and its Error type must match the specific S::Error
    Dispatcher<S>: CreateSecret<<S as SecretStore>::Error>,
{
    // todo: add logs here
    // todo: add metrics here

    let data = params.base64_encrypted_secret.as_bytes().to_owned();

    let secret = model::secret::Secret::new(data);

    info!("Received a new secret: {}", params.base64_encrypted_secret);

    let r = dispatcher.create_secret(secret).await;
    if let Err(e) = r {
        error!("Failed to create a secret because: {e}");
    }

    // if let Err(e) = service::methods::create_secret::create_secret(&mut storage, secret) {
    //     error!("failed to create and store the new secret because: {e}");
    // };

    StatusCode::OK
}
