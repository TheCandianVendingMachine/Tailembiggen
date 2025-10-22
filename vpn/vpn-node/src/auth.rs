mod domain;
mod error;
mod payloads;
mod session;

use std::fmt;
use rsa::rand_core::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};

pub use crate::auth::domain::{ Domain, NewDomain };
pub use error::Error;

#[derive(Clone)]
struct Key {
    public_key: RsaPublicKey,
    private_key: RsaPrivateKey,
}

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.public_key.fmt(f)
    }
}

impl Key {
    fn generate() -> Result<Key, Error> {
        let mut rng = OsRng {};
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits)?;
        let public_key = RsaPublicKey::from(&private_key);
        Ok(Key {
            public_key,
            private_key,
        })
    }
}

#[derive(Debug)]
pub struct Client {
    session: session::Session,
}

impl Client {
    pub fn from_session(session: session::Session) -> Client {
        Client { session }
    }
}

impl From<session::Session> for Client {
    fn from(session: session::Session) -> Client {
        Client::from_session(session)
    }
}
