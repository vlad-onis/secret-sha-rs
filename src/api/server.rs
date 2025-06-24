use std::sync::Arc;

use crate::{
    api,
    service::{dispatcher::Dispatcher, storage::in_memory_store::InMemorySecretStore},
};

use axum::{Router, routing::post};
use thiserror::Error;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use tracing::trace;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

pub fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        // Use the default formatting
        .with_max_level(tracing::Level::INFO) // Default log level if RUST_LOG is not set
        .with_env_filter(EnvFilter::from_default_env()) // Allow filtering by env var
        .finish();

    // 2. Set the subscriber as the global default.
    // This will route all `tracing` events through this subscriber.
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    trace!("Tracing init successfully");
}

pub async fn run() -> Result<(), Error> {
    init_tracing();
    let router = register_routes();

    // todo: values should come from configs, later
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await?;
    axum::serve(listener, router).await?;

    Ok(())
}

fn register_routes() -> Router {
    let router = Router::new();

    let storage = InMemorySecretStore::new();
    let dispatcher = Dispatcher::new(Arc::new(storage));

    router
        .route("/secrets/new", post(api::new_secret::handle))
        .with_state(dispatcher)
}
