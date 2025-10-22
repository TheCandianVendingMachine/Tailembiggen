pub mod create_account;
pub mod login;

use crate::auth::Error;
use crate::auth::domain::{ Domain, NewDomain };

#[derive(Debug)]
pub enum SessionState {
    Ok(Session),
    Expired(Expired),
}

#[derive(Debug, Clone)]
pub struct Expired {
    session: Session,
}

impl Expired {
    pub async fn refresh(self) -> Result<Session, Error> {
        Ok(self.session)
    }
}

#[derive(Debug, Clone)]
pub struct Session {
    pub domain: Domain,
    session_token: String,
    refresh_token: String,
}

impl Session {
}
