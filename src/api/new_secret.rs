use std::collections::HashMap;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use tracing::{error, info};

use crate::{
    model,
    service::{
        self, dispatcher::Dispatcher, methods::create_secret::CreateSecret,
        storage::store::InMemorySecretStore,
    },
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

pub async fn handle(
    State(dispatcher): State<Dispatcher>,
    Json(params): Json<NewSecretParams>,
) -> impl IntoResponse {
    // todo: add logs here
    // todo: add metrics here

    let data = params.base64_encrypted_secret.as_bytes().to_owned();

    let secret = model::secret::Secret::new(data);

    info!("Received a new secret: {}", params.base64_encrypted_secret);

    dispatcher.create_secret(secret).await.unwrap();

    // if let Err(e) = service::methods::create_secret::create_secret(&mut storage, secret) {
    //     error!("failed to create and store the new secret because: {e}");
    // };

    StatusCode::OK
}
