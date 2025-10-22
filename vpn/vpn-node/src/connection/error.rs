use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Could not connect: {0}")]
    SocketCannotConnect(#[from] std::io::Error),
}
