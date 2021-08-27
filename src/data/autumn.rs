use serde::{Deserialize, Serialize};

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
    Icons,
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
