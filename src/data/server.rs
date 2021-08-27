use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::data::autumn;

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: String,
    pub title: String,
    pub channels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemMessages {
    pub user_joined: Option<String>,
    pub user_left: Option<String>,
    pub user_kicked: Option<String>,
    pub user_banned: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Role {
    pub name: String,
    pub permissions: (i32, i32),
    pub colour: Option<String>,
    pub hoist: Option<bool>,
    pub rank: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    #[serde(rename = "_id")]
    pub id: String,
    pub nonce: Option<String>,
    pub owner: String,
    pub name: String,
    pub description: Option<String>,
    pub channels: Vec<String>,
    pub categories: Option<Vec<Category>>,
    pub system_messages: Option<SystemMessages>,
    pub roles: Option<HashMap<String, Role>>,
    pub default_permissions: (i32, i32),
    pub icon: Option<autumn::Attachment>,
    pub banner: Option<autumn::Attachment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemberId {
    pub server: String,
    pub user: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    #[serde(rename = "_id")]
    pub id: MemberId,
    pub nickname: Option<String>,
    pub avatar: Option<autumn::Attachment>,
    pub roles: Option<Vec<String>>,
}