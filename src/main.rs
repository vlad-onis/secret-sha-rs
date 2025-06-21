pub mod api;
pub mod model;
pub mod service;

use api::server::{Error as ServerError, run};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An api server error has occured: {0}")]
    Server(#[from] ServerError),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run().await?;

    Ok(())
}
