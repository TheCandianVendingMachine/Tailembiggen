mod domain;
mod error;
mod payloads;
mod session;

use rsa::rand_core::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};

pub use error::Error;
pub use session::create_account::CreateAccount;
pub use session::login::Login;

struct Key {
    public_key: RsaPublicKey,
    private_key: RsaPrivateKey,
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

pub struct Client {
    session: session::Session,
    key: Key,
}

impl Client {
    pub fn from_session(session: session::Session) -> Client {
        Client { session }
    }

    pub fn refresh_keys(&mut self) {}
}
