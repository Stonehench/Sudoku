pub mod streak;
pub mod auth;

#[derive(Debug,serde::Deserialize)]
pub struct UserInfo {
    user_id: String
}

#[derive(Debug,serde::Deserialize)]
pub struct OptionalUserInfo {
    user_id: Option<String>
}

pub mod scoreboard;