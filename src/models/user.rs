
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct User {
    pub id: i64,
    pub password: String,
    pub email: String,
    pub salt: String,
    pub last_login: Option<chrono::NaiveDateTime>,
    pub enable: bool,
    pub superuser: bool
}