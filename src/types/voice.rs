use super::JsonObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Voice {
    pub voice_id: String,
    pub voice_name: String,
    pub provider: String,
    pub gender: String,
    #[serde(default)]
    pub accent: Option<String>,
    #[serde(default)]
    pub age: Option<String>,
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub preview_audio_url: Option<String>,
    #[serde(flatten)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, Serialize)]
pub struct CloneVoiceParams {
    pub voice_name: String,
    #[serde(flatten)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddCommunityVoiceParams {
    pub voice_id: String,
    #[serde(flatten)]
    pub extra: JsonObject,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchCommunityVoiceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(flatten)]
    pub extra: JsonObject,
}
