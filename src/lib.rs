#![forbid(unsafe_code)]
use reqwest::header;
use rusty_ulid::generate_ulid_string;
use thiserror::Error;

pub mod data {
    pub mod autumn;
    pub mod server;
    pub mod channel;
    pub mod internal;
    pub mod message;
    pub mod user;
}

use crate::data::{channel, internal, message, user};

pub struct RevoltRs {
    pub client: reqwest::Client,
}

#[derive(Error, Debug)]
pub enum RevoltRsError {
    #[error("HTTP request failed")]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },
    #[error("parsing JSON into a struct failed")]
    Serde {
        #[from]
        source: serde_json::Error,
    },
    #[error("got an invalid header value")]
    Header {
        #[from]
        source: reqwest::header::InvalidHeaderValue,
    },
}

const API_URL: &str = "https://api.revolt.chat";

impl RevoltRs {
    pub fn new(token: &str) -> Result<Self, RevoltRsError> {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-String-TOKEN", header::HeaderValue::from_str(token)?);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(RevoltRs { client: client })
    }
    pub async fn fetch_user(self, user_id: &str) -> Result<user::User, RevoltRsError> {
        let url = format!("{0}/users/{1}", API_URL, user_id);
        let data = self.client.get(url).send().await?.text().await?;
        let user: user::User = serde_json::from_str(&data)?;
        Ok(user)
    }
    pub async fn edit_user(
        self,
        status: Option<user::Status>,
        profile: Option<user::Profile>,
        avatar: Option<String>,
        remove: Option<String>,
    ) -> Result<String, RevoltRsError> {
        let parameters = internal::EditUser {
            status: status,
            profile: profile,
            avatar: avatar,
            remove: remove,
        };
        let url = format!("{0}/users/@me", API_URL);
        let data = self
            .client
            .patch(url)
            .body(serde_json::to_string(&parameters)?)
            .send()
            .await?
            .text()
            .await?;
        Ok(data)
    }
    pub async fn fetch_user_profile(self, user_id: &str) -> Result<user::Profile, RevoltRsError> {
        let url = format!("{0}/users/{1}/profile", API_URL, user_id);
        let data = self.client.get(url).send().await?.text().await?;
        let profile: user::Profile = serde_json::from_str(&data)?;
        Ok(profile)
    }
    pub async fn fetch_default_avatar(self, user_id: &str) -> Result<Vec<u8>, RevoltRsError> {
        let url = format!("{0}/users/{1}/default_avatar", API_URL, user_id);
        let data = self
            .client
            .get(url)
            .send()
            .await?
            .text()
            .await?
            .into_bytes();
        Ok(data)
    }
    pub async fn fetch_mutual_friends(self, user_id: &str) -> Result<Vec<String>, RevoltRsError> {
        let url = format!("{0}/users/{1}/mutual", API_URL, user_id);
        let data = self.client.get(url).send().await?.text().await?;
        let mutual: Vec<String> = serde_json::from_str(&data)?;
        Ok(mutual)
    }
    pub async fn fetch_direct_message_channels(self) -> Result<channel::DMChannel, RevoltRsError> {
        let url = format!("{0}/users/dms", API_URL);
        let data = self.client.get(url).send().await?.text().await?;
        let dm_channel: channel::DMChannel = serde_json::from_str(&data)?;
        Ok(dm_channel)
    }
    pub async fn open_direct_message(
        self,
        user_id: &str,
    ) -> Result<channel::DMChannel, RevoltRsError> {
        let url = format!("{0}/users/{1}/dm", API_URL, user_id);
        let data = self.client.get(url).send().await?.text().await?;
        let dm_channel: channel::DMChannel = serde_json::from_str(&data)?;
        Ok(dm_channel)
    }
    pub async fn fetch_relationships(self) -> Result<Vec<user::Relationship>, RevoltRsError> {
        let url = format!("{0}/users/relationships", API_URL);
        let data = self.client.get(url).send().await?.text().await?;
        let relationships: Vec<user::Relationship> = serde_json::from_str(&data)?;
        Ok(relationships)
    }
    pub async fn fetch_relationship(
        self,
        user_id: &str,
    ) -> Result<user::Relationship, RevoltRsError> {
        let url = format!("{0}/users/{1}/relationship", API_URL, user_id);
        let data = self.client.get(url).send().await?.text().await?;
        let relationship: user::Relationship = serde_json::from_str(&data)?;
        Ok(relationship)
    }
    pub async fn send_accept_friend_request(
        self,
        username: &str,
    ) -> Result<user::Relationship, RevoltRsError> {
        let url = format!("{0}/users/{1}/friend", API_URL, username);
        let data = self.client.put(url).send().await?.text().await?;
        let relationship: user::Relationship = serde_json::from_str(&data)?;
        Ok(relationship)
    }
    pub async fn deny_remove_friend(
        self,
        username: &str,
    ) -> Result<user::Relationship, RevoltRsError> {
        let url = format!("{0}/users/{1}/friend", API_URL, username);
        let data = self.client.delete(url).send().await?.text().await?;
        let relationship: user::Relationship = serde_json::from_str(&data)?;
        Ok(relationship)
    }
    pub async fn block_user(self, user_id: &str) -> Result<user::Relationship, RevoltRsError> {
        let url = format!("{0}/users/{1}/block", API_URL, user_id);
        let data = self.client.put(url).send().await?.text().await?;
        let relationship: user::Relationship = serde_json::from_str(&data)?;
        Ok(relationship)
    }
    pub async fn unblock_user(self, user_id: &str) -> Result<user::Relationship, RevoltRsError> {
        let url = format!("{0}/users/{1}/block", API_URL, user_id);
        let data = self.client.delete(url).send().await?.text().await?;
        let relationship: user::Relationship = serde_json::from_str(&data)?;
        Ok(relationship)
    }
    pub async fn fetch_channel(self, channel_id: &str) -> Result<channel::Channel, RevoltRsError> {
        let url = format!("{0}/channels/{1}", API_URL, channel_id);
        let data = self.client.get(url).send().await?.text().await?;
        let channel: channel::Channel = serde_json::from_str(&data)?;
        Ok(channel)
    }
    pub async fn edit_channel(
        self,
        channel_id: &str,
        name: Option<String>,
        description: Option<String>,
        icon: Option<String>,
        remove: Option<String>,
    ) -> Result<String, RevoltRsError> {
        let url = format!("{0}/channels/{1}", API_URL, channel_id);
        let parameters = internal::EditAutumn {
            name: name,
            description: description,
            icon: icon,
            remove: remove,
        };
        let data = self
            .client
            .patch(url)
            .body(serde_json::to_string(&parameters)?)
            .send()
            .await?
            .text()
            .await?;
        Ok(data)
    }
    pub async fn close_channel(self, channel_id: &str) -> Result<String, RevoltRsError> {
        let url = format!("{0}/channels/{1}", API_URL, channel_id);
        let data = self.client.delete(url).send().await?.text().await?;
        Ok(data)
    }
    pub async fn create_invite(self, channel_id: &str) -> Result<String, RevoltRsError> {
        let url = format!("{0}/channels/{1}/invites", API_URL, channel_id);
        let data = self.client.post(url).send().await?.text().await?;
        let invite_struct: internal::Invite = serde_json::from_str(&data)?;
        let invite = invite_struct.code;
        Ok(invite)
    }
    pub async fn set_role_permission(
        self,
        channel_id: &str,
        role_id: &str,
        permissions: i32,
    ) -> Result<String, RevoltRsError> {
        let url = format!(
            "{0}/channels/{1}/permissions/{2}",
            API_URL, channel_id, role_id
        );
        let permission_struct = internal::Permissions {
            permissions: permissions,
        };
        let data = self
            .client
            .post(url)
            .body(serde_json::to_string(&permission_struct)?)
            .send()
            .await?
            .text()
            .await?;
        Ok(data)
    }
    pub async fn set_default_permission(
        self,
        channel_id: &str,
        permissions: i32,
    ) -> Result<String, RevoltRsError> {
        let url = format!("{0}/channels/{1}/permissions/default", API_URL, channel_id);
        let permission_struct = internal::Permissions {
            permissions: permissions,
        };
        let data = self
            .client
            .post(url)
            .body(serde_json::to_string(&permission_struct)?)
            .send()
            .await?
            .text()
            .await?;
        Ok(data)
    }
    pub async fn send_message(
        self,
        channel_id: &str,
        content: &str,
        attachments: Option<Vec<String>>,
        replies: Option<message::Reply>,
    ) -> Result<message::Message, RevoltRsError> {
        let url = format!("{0}/channels/{1}/messages", API_URL, channel_id);
        let parameters = internal::SendMessage {
            content: content.to_string(),
            nonce: generate_ulid_string(),
            attachments: attachments,
            replies: replies,
        };
        let data = self
            .client
            .post(url)
            .body(serde_json::to_string(&parameters)?)
            .send()
            .await?
            .text()
            .await?;
        let my_message = serde_json::from_str(&data)?;
        Ok(my_message)
    }
    pub async fn fetch_messages(
        self,
        channel_id: &str,
        limit: Option<i32>,
        before: Option<String>,
        after: Option<String>,
        sort: message::SearchSort,
        nearby: Option<String>,
        include_users: Option<bool>,
    ) -> Result<message::Messages, RevoltRsError> {
        let url = format!("{0}/channels/{1}/messages", API_URL, channel_id);
        let parameters = internal::SearchMessages {
            query: None,
            limit: limit,
            before: before,
            after: after,
            sort: sort,
            nearby: nearby,
            include_users: include_users,
        };
        let data = self
            .client
            .get(url)
            .body(serde_json::to_string(&parameters)?)
            .send()
            .await?
            .text()
            .await?;
        let messages: message::Messages = serde_json::from_str(&data)?;
        Ok(messages)
    }
    pub async fn fetch_message(
        self,
        channel_id: &str,
        message_id: &str,
    ) -> Result<message::Message, RevoltRsError> {
        let url = format!("{0}/channels/{1}/messages/{2}", API_URL, channel_id, message_id);
        let data = self
            .client
            .get(url)
            .send()
            .await?
            .text()
            .await?;
        let message: message::Message = serde_json::from_str(&data)?;
        Ok(message)
    }
    pub async fn edit_message(
        self,
        channel_id: &str,
        message_id: &str,
        content: &str,
    ) -> Result<String, RevoltRsError> {
        let url = format!("{0}/channels/{1}/messages/{2}", API_URL, channel_id, message_id);
        let parameters = internal::EditMessage {
            content: content.to_string(),
        };
        let data = self
            .client
            .patch(url)
            .body(serde_json::to_string(&parameters)?)
            .send()
            .await?
            .text()
            .await?;
        Ok(data)
    }
    pub async fn delete_message(
        self,
        channel_id: &str,
        message_id: &str,
    ) -> Result<String, RevoltRsError> {
        let url = format!("{0}/channels/{1}/messages/{2}", API_URL, channel_id, message_id);
        let data = self
            .client
            .delete(url)
            .send()
            .await?
            .text()
            .await?;
        Ok(data)
    }
}
