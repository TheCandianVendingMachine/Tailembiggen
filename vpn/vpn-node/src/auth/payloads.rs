use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
struct AuthAttempt {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
struct AuthResponse {
    pub session_token: String,
    pub refresh_token: String,
}
