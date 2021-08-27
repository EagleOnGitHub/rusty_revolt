use crate::data::{autumn, user, server};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LastMessage {
    #[serde(rename = "_id")]
    pub id: String,
    pub author: String,
    pub short: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LastMessageType {
    A(LastMessage),
    B(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageContent {
    #[serde(rename = "type")]
    pub message_type: String,
    pub name: Option<String>,
    pub id: Option<String>,
    pub by: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ContentType {
    A(MessageContent),
    B(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEdited {
    #[serde(rename = "$date")]
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum EmbedTypes {
    None,
    Website,
    Image,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SpecialEmbedTypes {
    None,
    YouTube,
    Twitch,
    Spotify,
    Soundcloud,
    Bandcamp,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SEContentTypes {
    Channel,
    Clip,
    Video,
    Album,
    Track,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SEContentType {
    A(SEContentTypes),
    B(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecialEmbed {
    #[serde(rename = "type")]
    pub se_type: SpecialEmbedTypes,
    pub content_type: Option<SEContentType>,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ImageSize {
    Large,
    Preview,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub url: String,
    pub width: i32,
    pub height: i32,
    pub size: ImageSize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Embed {
    #[serde(rename = "type")]
    pub embed_type: SpecialEmbedTypes,
    pub url: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub size: Option<ImageSize>,
    pub special: Option<SpecialEmbed>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<Image>,
    pub video: Option<Video>,
    pub site_name: Option<String>,
    pub icon_url: Option<String>,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: String,
    pub nonce: Option<String>,
    pub channel: String,
    pub author: String,
    pub content: Option<ContentType>,
    pub attachments: Option<Vec<autumn::Attachment>>,
    pub edited: Option<MessageEdited>,
    pub embeds: Option<Vec<Embed>>,
    pub mentions: Option<Vec<String>>,
    pub replies: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Messages {
    pub messages: Option<Vec<Message>>,
    pub users: Option<Vec<user::User>>,
    pub members: Option<Vec<server::Member>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reply {
    pub id: String,
    pub mention: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SearchSort {
    Latest,
    Oldest,
    Relevance,
}