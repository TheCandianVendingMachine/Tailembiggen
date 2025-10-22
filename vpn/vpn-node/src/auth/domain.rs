use crate::auth::Error;
use crate::auth::Key;
use crate::auth::session::{create_account::CreateAccount, login::Login};

#[derive(Debug, Clone)]
pub struct NewDomain {
    name: String,
}

#[derive(Debug, Clone)]
pub struct Domain {
    name: String,
    key: Key,
}

impl NewDomain {
    pub fn new<T>(domain: T) -> NewDomain
    where
        T: Into<String>,
    {
        NewDomain {
            name: domain.into(),
        }
    }

    pub fn create_account(&self) -> CreateAccount {
        CreateAccount::domain(self.clone())
    }
}

impl Domain {
    pub fn login(&self) -> Login {
        Login::domain(self.clone())
    }
}

impl TryFrom<NewDomain> for Domain {
    type Error = Error;
    fn try_from(domain: NewDomain) -> Result<Self, Self::Error> {
        Ok(Domain {
            name: domain.name,
            key: Key::generate()?,
        })
    }
}
