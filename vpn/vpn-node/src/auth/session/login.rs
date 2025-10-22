use crate::auth::error::Error;
use crate::auth::session::Domain;
use crate::auth::session::Session;

#[derive(Debug, Clone)]
pub struct Login {
    domain: Domain,
}

#[derive(Debug, Clone)]
pub struct LoginWithUsername {
    login: Login,
    username: String,
}

impl Login {
    pub fn domain(domain: Domain) -> Login {
        Login { domain }
    }

    pub fn username(self, username: String) -> LoginWithUsername {
        LoginWithUsername {
            login: self,
            username,
        }
    }
}

impl LoginWithUsername {
    pub async fn login<T>(self, password: T) -> Result<Session, Error>
    where
        T: Into<String>,
    {
        Ok(Session {
            domain: self.login.domain,
            session_token: String::new(),
            refresh_token: String::new(),
        })
    }
}
