use crate::auth::domain::Domain;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Could not create auth key")]
    AuthKeyNotGenerated(#[from] rsa::Error),
    #[error("Login to VPN authenticator failed")]
    LoginFailed,
    #[error("Failed to refresh session")]
    RefreshFailed,
    #[error("Could not create account")]
    AccountCreationError,
    #[error("Domain \"{domain:?}\" does not exist")]
    UnknownDomain { domain: Domain },
}
