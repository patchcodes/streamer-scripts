use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscordMessageResponse {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub content: String,
    pub mentions: Vec<Value>,
    #[serde(rename = "mention_roles")]
    pub mention_roles: Vec<Value>,
    pub attachments: Vec<Value>,
    pub embeds: Vec<Value>,
    pub timestamp: String,
    #[serde(rename = "edited_timestamp")]
    pub edited_timestamp: Value,
    pub flags: i64,
    pub components: Vec<Value>,
    pub id: String,
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    pub author: Author,
    pub pinned: bool,
    #[serde(rename = "mention_everyone")]
    pub mention_everyone: bool,
    pub tts: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: String,
    pub username: String,
    pub avatar: Value,
    pub discriminator: String,
    #[serde(rename = "public_flags")]
    pub public_flags: i64,
    pub flags: i64,
    pub bot: bool,
    pub banner: Value,
    #[serde(rename = "accent_color")]
    pub accent_color: Value,
    #[serde(rename = "global_name")]
    pub global_name: Value,
    #[serde(rename = "avatar_decoration_data")]
    pub avatar_decoration_data: Value,
    pub collectibles: Value,
    #[serde(rename = "banner_color")]
    pub banner_color: Value,
    pub clan: Value,
    #[serde(rename = "primary_guild")]
    pub primary_guild: Value,
}
