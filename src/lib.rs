use reqwest::header;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
pub struct Attachment {
    pub _id: String,
    pub tag: String,
    pub size: i32,
    pub filename: String,
    pub metadata: Metadata,
    pub content_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationship {
    pub status: String,
    pub _id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub text: Option<String>,
    pub presence: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub _id: String,
    pub username: String,
    pub avatar: Option<Attachment>,
    pub relations: Option<Vec<Relationship>>,
    pub badges: Option<i32>,
    pub status: Option<Status>,
    pub relationship: Option<String>,
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

#[derive(Serialize, Deserialize)]
pub struct EditUser {
    pub status: Option<Status>,
    pub profile: Option<Profile>,
    pub avatar: Option<String>,
    pub remove: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LastMessage {
    pub _id: String,
    pub author: String,
    pub short: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectMessageChannel {
    pub _id: String,
    pub channel_type: String,
    pub active: Option<bool>,
    pub recipients: Vec<String>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub description: Option<String>,
    pub last_message: LastMessage,
    pub icon: Option<Attachment>,
    pub permissions: Option<i32>
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
    }
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
    pub async fn edit_user(self, argument: EditUser) -> Result<(), RevoltRsError> {
        self.client.patch("https://api.revolt.chat/users/@me")
            .body(serde_json::to_string(&argument)?)
            .send().await?;
        Ok(())
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
}