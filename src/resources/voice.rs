use crate::error::Result;
use crate::http::{enc, HttpClient};
use crate::types::{AddCommunityVoiceParams, CloneVoiceParams, SearchCommunityVoiceParams, Voice};
use reqwest::Method;
use serde_json::Value;

/// Voice catalog operations (`client.voice()`).
#[derive(Clone)]
pub struct VoiceClient {
    pub(crate) http: HttpClient,
}

impl VoiceClient {
    /// `GET /list-voices`
    pub async fn list(&self) -> Result<Vec<Voice>> {
        self.http.send_empty(Method::GET, "/list-voices").await
    }

    /// `GET /get-voice/{voice_id}`
    pub async fn get(&self, voice_id: &str) -> Result<Voice> {
        self.http
            .send_empty(Method::GET, &format!("/get-voice/{}", enc(voice_id)))
            .await
    }

    /// `POST /clone-voice`
    pub async fn clone_voice(&self, params: &CloneVoiceParams) -> Result<Voice> {
        self.http
            .send_json(Method::POST, "/clone-voice", params)
            .await
    }

    /// `POST /add-community-voice`
    pub async fn add_community(&self, params: &AddCommunityVoiceParams) -> Result<Voice> {
        self.http
            .send_json(Method::POST, "/add-community-voice", params)
            .await
    }

    /// `POST /search-community-voice`
    pub async fn search_community(&self, params: &SearchCommunityVoiceParams) -> Result<Value> {
        self.http
            .send_json(Method::POST, "/search-community-voice", params)
            .await
    }
}
