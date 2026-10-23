use crate::error::Result;
use crate::http::{enc, HttpClient};
use crate::types::{CreateRetellLlmParams, RetellLlm, UpdateRetellLlmParams};
use reqwest::Method;

/// Retell LLM response-engine operations (`client.llm()`).
#[derive(Clone)]
pub struct LlmClient {
    pub(crate) http: HttpClient,
}

impl LlmClient {
    /// `POST /create-retell-llm`
    pub async fn create(&self, params: &CreateRetellLlmParams) -> Result<RetellLlm> {
        self.http
            .send_json(Method::POST, "/create-retell-llm", params)
            .await
    }

    /// `GET /get-retell-llm/{llm_id}`
    pub async fn get(&self, llm_id: &str) -> Result<RetellLlm> {
        self.http
            .send_empty(Method::GET, &format!("/get-retell-llm/{}", enc(llm_id)))
            .await
    }

    /// `GET /v2/list-retell-llms`
    pub async fn list(&self) -> Result<Vec<RetellLlm>> {
        self.http
            .send_empty(Method::GET, "/v2/list-retell-llms")
            .await
    }

    /// `PATCH /update-retell-llm/{llm_id}`
    pub async fn update(&self, llm_id: &str, params: &UpdateRetellLlmParams) -> Result<RetellLlm> {
        self.http
            .send_json(
                Method::PATCH,
                &format!("/update-retell-llm/{}", enc(llm_id)),
                params,
            )
            .await
    }

    /// `DELETE /delete-retell-llm/{llm_id}`
    pub async fn delete(&self, llm_id: &str) -> Result<()> {
        self.http
            .send_no_content(
                Method::DELETE,
                &format!("/delete-retell-llm/{}", enc(llm_id)),
            )
            .await
    }
}
