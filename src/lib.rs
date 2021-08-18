#![forbid(unsafe_code)]
use reqwest::header;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::collections::HashMap;
use rusty_ulid::generate_ulid_string;

pub struct RevoltRs {
    client: reqwest::Client,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Metadata {
    File,
    Text,
    Audio,
    Image { width: i32, height: i32 },
    Video { width: i32, height: i32 },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum AttachmentTag {
    Attachments,
    Avatars,
    Backgrounds,
    Banners,
    Icons
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    #[serde(rename = "_id")]
    pub id: String,
    pub tag: AttachmentTag,
    pub size: i32,
    pub filename: String,
    pub metadata: Metadata,
    pub content_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum RelationshipStatus {
    Blocked,
    BlockedOther,
    Friend,
    Incoming,
    None,
    Outgoing,
    User
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationship {
    pub status: RelationshipStatus,
    #[serde(rename = "_id")]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StatusPresence {
    Busy,
    Idle,
    Invisible,
    Online
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub text: Option<String>,
    pub presence: Option<StatusPresence>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub avatar: Option<Attachment>,
    pub relations: Option<Vec<Relationship>>,
    pub badges: Option<i32>,
    pub status: Option<Status>,
    pub relationship: Option<RelationshipStatus>,
    pub online: Option<bool>,
    pub flags: Option<i32>,
    pub bot: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ProfileTypes {
    A(String),
    B(Attachment)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub content: Option<String>,
    pub background: Option<ProfileTypes>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EditUserRemove {
    Avatar,
    ProfileBackground,
    ProfileContent,
    StatusText
}

#[derive(Serialize, Deserialize)]
pub struct EditUser {
    pub status: Option<Status>,
    pub profile: Option<Profile>,
    pub avatar: Option<String>,
    pub remove: Option<EditUserRemove>
}

#[derive(Serialize, Deserialize)]
pub struct EditAutumn {
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub remove: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LastMessage {
    #[serde(rename = "_id")]
    pub id: String,
    pub author: String,
    pub short: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LastMessageType {
    A(LastMessage),
    B(String)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectMessageChannel {
    #[serde(rename = "_id")]
    pub id: String,
    pub channel_type: ChannelTypes,
    pub active: Option<bool>,
    pub recipients: Vec<String>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub description: Option<String>,
    pub last_message: LastMessage,
    pub icon: Option<Attachment>,
    pub permissions: Option<i32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RolePermissions {
    pub role_permissions: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ChannelTypes {
    SavedMessages,
    DirectMessage,
    Group,
    TextChannel,
    VoiceChannel
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
    pub last_message: Option<LastMessageType>,
    pub user: Option<String>,
    pub icon: Option<Attachment>,
    pub default_permissions: Option<i32>,
    pub role_permissions: Option<HashMap<String, i32>>,
    pub permissions: Option<i32>,
    pub nonce: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permissions {
    pub permissions: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Invite {
    pub code: String
}

#[derive(Error, Debug)]
pub enum RevoltRsError {
    #[error("HTTP request failed")]
    Reqwest {
        #[from]
        source: reqwest::Error
    },
    #[error("parsing JSON into a struct failed")]
    Serde {
        #[from]
        source: serde_json::Error
    },
}

// Base API URL: https://api.revolt.chat
impl RevoltRs {
    pub fn new(token: &'static str) -> Result<Self, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-String-TOKEN", header::HeaderValue::from_static(token));
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(RevoltRs { client: client })
    }
    pub async fn fetch_user(self, user_id: &str) -> Result<User, RevoltRsError> {
        let url = format!("https://api.revolt.chat/users/{0}", user_id);
        let data = self.client.get(url)
            .send().await?
            .text().await?;
        let user: User = serde_json::from_str(&data)?;
        Ok(user)
    }
    pub async fn edit_user(self, parameters: EditUser) -> Result<String, RevoltRsError> {
        let data = self.client.patch("https://api.revolt.chat/users/@me")
            .body(serde_json::to_string(&parameters)?)
            .send().await?
            .text().await?;
        Ok(data)
    }
    pub async fn fetch_user_profile(self, user_id: &str) -> Result<Profile, RevoltRsError> {
        let url = format!("https://api.revolt.chat/users/{0}/profile", user_id);
        let data = self.client.get(url)
            .send().await?
            .text().await?;
        let profile: Profile = serde_json::from_str(&data)?;
        Ok(profile)
    }
    pub async fn fetch_default_avatar(self, user_id: &str) -> Result<Vec<u8>, RevoltRsError> {
        let url = format!("https://api.revolt.chat/users/{0}/default_avatar", user_id);
        let data = self.client.get(url)
            .send().await?
            .text().await?
            .into_bytes();
        Ok(data)
    }
    pub async fn fetch_mutual_friends(self, user_id: &str) -> Result<Vec<String>, RevoltRsError> {
        let url = format!("https://api.revolt.chat/users/{0}/mutual", user_id);
        let data = self.client.get(url)
            .send().await?
            .text().await?;
        let mutual: Vec<String> = serde_json::from_str(&data)?;
        Ok(mutual)
    }
    pub async fn fetch_direct_message_channels(self) -> Result<DirectMessageChannel, RevoltRsError> {
        let data = self.client.get("https://api.revolt.chat/users/dms")
            .send().await?
            .text().await?;
        let dm_channel: DirectMessageChannel = serde_json::from_str(&data)?;
        Ok(dm_channel)
    }
    pub async fn open_direct_message(self, user_id: &str) -> Result<DirectMessageChannel, RevoltRsError> {
        let url = format!("https://api.revolt.chat/users/{0}/dm", user_id);
        let data = self.client.get(url)
            .send().await?
            .text().await?;
        let dm_channel: DirectMessageChannel = serde_json::from_str(&data)?;
        Ok(dm_channel)
    }
    pub async fn fetch_relationships(self) -> Result<Vec<Relationship>, RevoltRsError> {
        let data = self.client.get("https://api.revolt.chat/users/relationships")
            .send().await?
            .text().await?;
        let relationships: Vec<Relationship> = serde_json::from_str(&data)?;
        Ok(relationships)
    }
    pub async fn fetch_relationship(self, user_id: &str) -> Result<Relationship, RevoltRsError> {
        let url = format!("https://api.revolt.chat/users/{0}/relationship", user_id);
        let data = self.client.get(url)
            .send().await?
            .text().await?;
        let relationship: Relationship = serde_json::from_str(&data)?;
        Ok(relationship)
    }
    pub async fn send_accept_friend_request(self, username: &str) -> Result<Relationship, RevoltRsError> {
        let url = format!("https://api.revolt.chat/users/{0}/friend", username);
        let data = self.client.put(url)
            .send().await?
            .text().await?;
        let relationship: Relationship = serde_json::from_str(&data)?;
        Ok(relationship)
    }
    pub async fn deny_remove_friend(self, username: &str) -> Result<Relationship, RevoltRsError> {
        let url = format!("https://api.revolt.chat/users/{0}/friend", username);
        let data = self.client.delete(url)
            .send().await?
            .text().await?;
        let relationship: Relationship = serde_json::from_str(&data)?;
        Ok(relationship)
    }
    pub async fn block_user(self, user_id: &str) -> Result<Relationship, RevoltRsError> {
        let url = format!("https://api.revolt.chat/users/{0}/block", user_id);
        let data = self.client.put(url)
            .send().await?
            .text().await?;
        let relationship: Relationship = serde_json::from_str(&data)?;
        Ok(relationship)
    }
    pub async fn unblock_user(self, user_id: &str) -> Result<Relationship, RevoltRsError> {
        let url = format!("https://api.revolt.chat/users/{0}/block", user_id);
        let data = self.client.delete(url)
            .send().await?
            .text().await?;
        let relationship: Relationship = serde_json::from_str(&data)?;
        Ok(relationship)
    }
    pub async fn fetch_channel(self, channel_id: &str) -> Result<Channel, RevoltRsError> {
        let url = format!("https://api.revolt.chat/channels/{0}", channel_id);
        let data = self.client.get(url)
            .send().await?
            .text().await?;
        let channel: Channel = serde_json::from_str(&data)?;
        Ok(channel)
    }
    pub async fn edit_channel(self, channel_id: &str, parameters: EditAutumn) -> Result<String, RevoltRsError> {
        let url = format!("https://api.revolt.chat/channels/{0}", channel_id);
        let data = self.client.patch(url)
            .body(serde_json::to_string(&parameters)?)
            .send().await?
            .text().await?;
        Ok(data)
    }
    pub async fn close_channel(self, channel_id: &str) -> Result<String, RevoltRsError> {
        let url = format!("https://api.revolt.chat/channels/{0}", channel_id);
        let data = self.client.delete(url)
            .send().await?
            .text().await?;
        Ok(data)
    }
    pub async fn create_invite(self, channel_id: &str) -> Result<Invite, RevoltRsError> {
        let url = format!("https://api.revolt.chat/channels/{0}/invites", channel_id);
        let data = self.client.post(url)
            .send().await?
            .text().await?;
        let invite: Invite = serde_json::from_str(&data)?;
        Ok(invite)
    }
    pub async fn set_role_permission(self, channel_id: &str, role_id: &str, permissions: Permissions) -> Result<String, RevoltRsError> {
        let url = format!("https://api.revolt.chat/channels/{0}/permissions/{1}", channel_id, role_id);
        let data = self.client.post(url)
            .body(serde_json::to_string(&permissions)?)
            .send().await?
            .text().await?;
        Ok(data)
    }
    pub async fn set_default_permission(self, channel_id: &str, permissions: Permissions) -> Result<String, RevoltRsError> {
        let url = format!("https://api.revolt.chat/channels/{0}/permissions/default", channel_id);
        let data = self.client.post(url)
            .body(serde_json::to_string(&permissions)?)
            .send().await?
            .text().await?;
        Ok(data)
    }
}