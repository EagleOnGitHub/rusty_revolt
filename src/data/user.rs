use crate::data::autumn;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum RelationshipStatus {
    Blocked,
    BlockedOther,
    Friend,
    Incoming,
    None,
    Outgoing,
    User,
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
    Online,
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
    pub avatar: Option<autumn::Attachment>,
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
    B(autumn::Attachment),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub content: Option<String>,
    pub background: Option<ProfileTypes>,
}
