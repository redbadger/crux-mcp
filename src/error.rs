use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
#[error("{0}")]
pub enum Error {
    #[serde(skip)]
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
