use serde::{ Serialize, Deserialize };
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Login to VPN authenticator failed")]
    LoginFailed
}

#[derive(Debug, Clone, Serialize)]
struct AuthAttempt {
    pub username: String,
    pub password: String
}

#[derive(Debug, Clone, Deserialize)]
struct AuthResponse {
    pub session_token: String
}

#[derive(Debug, Clone)]
pub struct Login {
    username: String,
}

impl Login {
    pub fn username<T>(username: T) -> Login where T: Into<String> {
        let username = username.into();
        Login {
            username
        }
    }

    pub async fn login<T>(self, password: T) -> Result<Session, Error> where T: Into<String> {
        Ok(Session {})
    }
}

pub struct Session {
}
