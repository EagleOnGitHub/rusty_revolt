use crate::data::{message, user};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EditUser {
    pub status: Option<user::Status>,
    pub profile: Option<user::Profile>,
    pub avatar: Option<String>,
    pub remove: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EditAutumn {
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub remove: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permissions {
    pub permissions: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Invite {
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendMessage {
    pub content: String,
    pub nonce: String,
    pub attachments: Option<Vec<String>>,
    pub replies: Option<message::Reply>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchMessages {
    pub query: Option<String>,
    pub limit: Option<i32>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub sort: message::SearchSort,
    pub nearby: Option<String>,
    pub include_users: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct EditMessage {
    pub content: String,
}