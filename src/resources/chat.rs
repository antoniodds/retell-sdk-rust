use crate::error::Result;
use crate::http::{enc, HttpClient};
use crate::types::{
    Chat, CreateChatCompletionParams, CreateChatParams, CreateSmsChatParams, ListChatsParams,
    Paginated, UpdateChatParams,
};
use reqwest::Method;
use serde_json::Value;

/// Chat operations (`client.chat()`).
#[derive(Clone)]
pub struct ChatClient {
    pub(crate) http: HttpClient,
}

impl ChatClient {
    /// `POST /create-chat`
    pub async fn create(&self, params: &CreateChatParams) -> Result<Chat> {
        self.http
            .send_json(Method::POST, "/create-chat", params)
            .await
    }

    /// `POST /create-sms-chat`
    pub async fn create_sms(&self, params: &CreateSmsChatParams) -> Result<Chat> {
        self.http
            .send_json(Method::POST, "/create-sms-chat", params)
            .await
    }

    /// `GET /get-chat/{chat_id}`
    pub async fn get(&self, chat_id: &str) -> Result<Chat> {
        self.http
            .send_empty(Method::GET, &format!("/get-chat/{}", enc(chat_id)))
            .await
    }

    /// `POST /v3/list-chats`
    pub async fn list(&self, params: &ListChatsParams) -> Result<Paginated<Chat>> {
        self.http
            .send_json(Method::POST, "/v3/list-chats", params)
            .await
    }

    /// `DELETE /delete-chat/{chat_id}`
    pub async fn delete(&self, chat_id: &str) -> Result<()> {
        self.http
            .send_no_content(Method::DELETE, &format!("/delete-chat/{}", enc(chat_id)))
            .await
    }

    /// `PATCH /end-chat/{chat_id}`
    pub async fn end(&self, chat_id: &str) -> Result<Chat> {
        self.http
            .send_empty(Method::PATCH, &format!("/end-chat/{}", enc(chat_id)))
            .await
    }

    /// `PATCH /update-chat/{chat_id}`
    pub async fn update(&self, chat_id: &str, params: &UpdateChatParams) -> Result<Chat> {
        self.http
            .send_json(
                Method::PATCH,
                &format!("/update-chat/{}", enc(chat_id)),
                params,
            )
            .await
    }

    /// `POST /create-chat-completion`
    pub async fn create_completion(&self, params: &CreateChatCompletionParams) -> Result<Value> {
        self.http
            .send_json(Method::POST, "/create-chat-completion", params)
            .await
    }

    /// `PUT /rerun-chat-analysis/{chat_id}`
    pub async fn rerun_analysis(&self, chat_id: &str) -> Result<()> {
        self.http
            .send_no_content(
                Method::PUT,
                &format!("/rerun-chat-analysis/{}", enc(chat_id)),
            )
            .await
    }
}
