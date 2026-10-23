use crate::error::Result;
use crate::http::{enc, HttpClient};
use crate::types::{
    Agent, AgentListItem, ChatAgent, CreateAgentParams, CreateChatAgentParams, ListAgentsParams,
    ListParams, Paginated, UpdateAgentParams,
};
use reqwest::Method;
use serde_json::Value;

/// Voice & chat agent operations (`client.agent()`).
#[derive(Clone)]
pub struct AgentClient {
    pub(crate) http: HttpClient,
}

impl AgentClient {
    /// `POST /create-agent`
    pub async fn create(&self, params: &CreateAgentParams) -> Result<Agent> {
        self.http
            .send_json(Method::POST, "/create-agent", params)
            .await
    }

    /// `GET /get-agent/{agent_id}`
    pub async fn get(&self, agent_id: &str) -> Result<Agent> {
        self.http
            .send_empty(Method::GET, &format!("/get-agent/{}", enc(agent_id)))
            .await
    }

    /// `GET /get-agent/{agent_id}?version=`
    pub async fn get_version(&self, agent_id: &str, version: i64) -> Result<Agent> {
        self.http
            .send_query(
                Method::GET,
                &format!("/get-agent/{}", enc(agent_id)),
                &[("version", version.to_string())],
            )
            .await
    }

    /// `PATCH /update-agent/{agent_id}`
    pub async fn update(&self, agent_id: &str, params: &UpdateAgentParams) -> Result<Agent> {
        self.http
            .send_json(
                Method::PATCH,
                &format!("/update-agent/{}", enc(agent_id)),
                params,
            )
            .await
    }

    /// `DELETE /delete-agent/{agent_id}`
    pub async fn delete(&self, agent_id: &str) -> Result<()> {
        self.http
            .send_no_content(Method::DELETE, &format!("/delete-agent/{}", enc(agent_id)))
            .await
    }

    /// `POST /v2/list-agents`
    pub async fn list(
        &self,
        list: &ListParams,
        body: &ListAgentsParams,
    ) -> Result<Paginated<AgentListItem>> {
        self.http
            .send_json_query(Method::POST, "/v2/list-agents", body, &list.as_query())
            .await
    }

    /// Convenience: list first page with defaults.
    pub async fn list_all(&self) -> Result<Paginated<AgentListItem>> {
        self.list(&ListParams::new(), &ListAgentsParams::default())
            .await
    }

    /// `POST /create-agent-version/{agent_id}`
    pub async fn create_version(&self, agent_id: &str, params: &Value) -> Result<Agent> {
        self.http
            .send_json(
                Method::POST,
                &format!("/create-agent-version/{}", enc(agent_id)),
                params,
            )
            .await
    }

    /// `GET /list-agent-versions/{agent_id}`
    pub async fn list_versions(&self, agent_id: &str) -> Result<Value> {
        self.http
            .send_empty(
                Method::GET,
                &format!("/list-agent-versions/{}", enc(agent_id)),
            )
            .await
    }

    /// `POST /publish-agent-version/{agent_id}`
    pub async fn publish(&self, agent_id: &str, body: &Value) -> Result<Value> {
        self.http
            .send_json(
                Method::POST,
                &format!("/publish-agent-version/{}", enc(agent_id)),
                body,
            )
            .await
    }

    /// `DELETE /delete-agent-version/{agent_id}`
    pub async fn delete_version(&self, agent_id: &str, version: i64) -> Result<()> {
        self.http
            .send_query_no_content(
                Method::DELETE,
                &format!("/delete-agent-version/{}", enc(agent_id)),
                &[("version", version.to_string())],
            )
            .await
    }

    /// `POST /repair-agent/{agent_id}`
    pub async fn repair(&self, agent_id: &str) -> Result<Value> {
        self.http
            .send_empty(Method::POST, &format!("/repair-agent/{}", enc(agent_id)))
            .await
    }

    // --- Chat agents ---

    /// `POST /create-chat-agent`
    pub async fn create_chat_agent(&self, params: &CreateChatAgentParams) -> Result<ChatAgent> {
        self.http
            .send_json(Method::POST, "/create-chat-agent", params)
            .await
    }

    /// `GET /get-chat-agent/{agent_id}`
    pub async fn get_chat_agent(&self, agent_id: &str) -> Result<ChatAgent> {
        self.http
            .send_empty(Method::GET, &format!("/get-chat-agent/{}", enc(agent_id)))
            .await
    }

    /// `PATCH /update-chat-agent/{agent_id}`
    pub async fn update_chat_agent(
        &self,
        agent_id: &str,
        params: &Value,
    ) -> Result<ChatAgent> {
        self.http
            .send_json(
                Method::PATCH,
                &format!("/update-chat-agent/{}", enc(agent_id)),
                params,
            )
            .await
    }

    /// `DELETE /delete-chat-agent/{agent_id}`
    pub async fn delete_chat_agent(&self, agent_id: &str) -> Result<()> {
        self.http
            .send_no_content(
                Method::DELETE,
                &format!("/delete-chat-agent/{}", enc(agent_id)),
            )
            .await
    }
}
