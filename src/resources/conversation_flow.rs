use crate::error::Result;
use crate::http::{enc, HttpClient};
use crate::types::{
    ConversationFlow, CreateConversationFlowParams, ListParams, Paginated,
    UpdateConversationFlowParams,
};
use reqwest::Method;
use serde_json::{json, Value};

/// Conversation flow operations (`client.conversation_flow()`).
#[derive(Clone)]
pub struct ConversationFlowClient {
    pub(crate) http: HttpClient,
}

impl ConversationFlowClient {
    /// `POST /create-conversation-flow`
    pub async fn create(
        &self,
        params: &CreateConversationFlowParams,
    ) -> Result<ConversationFlow> {
        self.http
            .send_json(Method::POST, "/create-conversation-flow", params)
            .await
    }

    /// `GET /get-conversation-flow/{conversation_flow_id}`
    pub async fn get(&self, conversation_flow_id: &str) -> Result<ConversationFlow> {
        self.http
            .send_empty(
                Method::GET,
                &format!("/get-conversation-flow/{}", enc(conversation_flow_id)),
            )
            .await
    }

    /// `GET /v2/list-conversation-flows`
    pub async fn list(&self, list: &ListParams) -> Result<Paginated<ConversationFlow>> {
        // Endpoint returns an array in some revisions; normalize via Value then re-parse.
        let raw: Value = self
            .http
            .send_query(
                Method::GET,
                "/v2/list-conversation-flows",
                &list.as_query(),
            )
            .await?;
        if raw.is_array() {
            let items: Vec<ConversationFlow> = serde_json::from_value(raw).map_err(crate::Error::Decode)?;
            Ok(Paginated {
                items,
                has_more: false,
                pagination_key: None,
                total: None,
            })
        } else {
            serde_json::from_value(raw).map_err(crate::Error::Decode)
        }
    }

    /// `PATCH /update-conversation-flow/{conversation_flow_id}`
    pub async fn update(
        &self,
        conversation_flow_id: &str,
        params: &UpdateConversationFlowParams,
    ) -> Result<ConversationFlow> {
        self.http
            .send_json(
                Method::PATCH,
                &format!("/update-conversation-flow/{}", enc(conversation_flow_id)),
                params,
            )
            .await
    }

    /// `DELETE /delete-conversation-flow/{conversation_flow_id}`
    pub async fn delete(&self, conversation_flow_id: &str) -> Result<()> {
        self.http
            .send_no_content(
                Method::DELETE,
                &format!("/delete-conversation-flow/{}", enc(conversation_flow_id)),
            )
            .await
    }

    /// `POST /create-conversation-flow-component`
    pub async fn create_component(&self, body: &Value) -> Result<Value> {
        self.http
            .send_json(Method::POST, "/create-conversation-flow-component", body)
            .await
    }

    /// `GET /get-conversation-flow-component/{id}`
    pub async fn get_component(&self, id: &str) -> Result<Value> {
        self.http
            .send_empty(
                Method::GET,
                &format!("/get-conversation-flow-component/{}", enc(id)),
            )
            .await
    }

    /// `GET /v2/list-conversation-flow-components`
    pub async fn list_components(&self, list: &ListParams) -> Result<Value> {
        self.http
            .send_query(
                Method::GET,
                "/v2/list-conversation-flow-components",
                &list.as_query(),
            )
            .await
    }

    /// `PATCH /update-conversation-flow-component/{id}`
    pub async fn update_component(&self, id: &str, body: &Value) -> Result<Value> {
        self.http
            .send_json(
                Method::PATCH,
                &format!("/update-conversation-flow-component/{}", enc(id)),
                body,
            )
            .await
    }

    /// `DELETE /delete-conversation-flow-component/{id}`
    pub async fn delete_component(&self, id: &str) -> Result<()> {
        self.http
            .send_no_content(
                Method::DELETE,
                &format!("/delete-conversation-flow-component/{}", enc(id)),
            )
            .await
    }

    /// Helper for empty create bodies.
    pub fn empty_body() -> Value {
        json!({})
    }
}
