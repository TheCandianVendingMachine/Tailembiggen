use crate::auth::Error;
use crate::auth::session::{create_account::CreateAccount, login::Login};

#[derive(Debug, Clone)]
pub struct Domain {
    name: String,
}

impl Domain {
    pub fn login(&self) -> Login {
        Login::domain(self.clone())
    }

    pub fn create_account(&self) -> Result<CreateAccount, Error> {
        CreateAccount::domain(self.clone())
    }
}
