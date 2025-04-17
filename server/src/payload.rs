use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequestBody {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponseBody {
    pub id: String,
    pub username: String,
}
