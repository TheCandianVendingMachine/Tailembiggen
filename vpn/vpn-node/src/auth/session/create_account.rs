use crate::auth::Error;
use crate::auth::session::{ NewDomain, Domain };
use crate::auth::session::Session;

pub struct CreateAccount {
    domain: NewDomain,
}

pub struct CreateAccountWithUsername {
    create_account: CreateAccount,
    username: String,
}

impl CreateAccount {
    pub fn domain(domain: NewDomain) -> CreateAccount {
        CreateAccount { domain }
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
            domain: Domain::try_from(self.create_account.domain)?,
            session_token: String::new(),
            refresh_token: String::new(),
        })
    }
}
