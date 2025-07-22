use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("channel closed")]
    ChannelClosed,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
