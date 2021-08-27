use crate::data::{autumn, message};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct DMChannel {
    #[serde(rename = "_id")]
    pub id: String,
    pub channel_type: ChannelTypes,
    pub active: Option<bool>,
    pub recipients: Vec<String>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub description: Option<String>,
    pub last_message: message::LastMessage,
    pub icon: Option<autumn::Attachment>,
    pub permissions: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RolePermissions {
    pub role_permissions: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ChannelTypes {
    SavedMessages,
    DirectMessage,
    Group,
    TextChannel,
    VoiceChannel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    #[serde(rename = "_id")]
    pub id: String,
    pub server: Option<String>,
    pub channel_type: ChannelTypes,
    pub active: Option<bool>,
    pub recipients: Option<Vec<String>>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub description: Option<String>,
    pub last_message: Option<message::LastMessageType>,
    pub user: Option<String>,
    pub icon: Option<autumn::Attachment>,
    pub default_permissions: Option<i32>,
    pub role_permissions: Option<HashMap<String, i32>>,
    pub permissions: Option<i32>,
    pub nonce: Option<String>,
}
