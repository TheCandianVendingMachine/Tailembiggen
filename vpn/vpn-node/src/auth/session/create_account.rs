use crate::auth::Error;
use crate::auth::Key;
use crate::auth::session::Domain;
use crate::auth::session::Session;

pub struct CreateAccount {
    domain: Domain,
    key: Key,
}

pub struct CreateAccountWithUsername {
    create_account: CreateAccount,
    username: String,
}

impl CreateAccount {
    pub fn domain(domain: Domain) -> Result<CreateAccount, Error> {
        let key = Key::generate()?;
        Ok(CreateAccount { domain, key })
    }

    pub fn username<T>(self, username: T) -> Result<CreateAccountWithUsername, Error>
    where
        T: Into<String>,
    {
        Ok(CreateAccountWithUsername {
            create_account: self,
            username: username.into(),
        })
    }
}

impl CreateAccountWithUsername {
    pub async fn password<T>(self, password: T) -> Result<Session, Error>
    where
        T: Into<String>,
    {
        let password = password.into();
        Ok(Session {
            domain: self.create_account.domain,
            session_token: String::new(),
            refresh_token: String::new(),
        })
    }
}
