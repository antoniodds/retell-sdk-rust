use crate::error::Result;
use crate::http::{enc, HttpClient};
use crate::types::{
    BatchCall, Call, CreateBatchCallParams, CreatePhoneCallParams, CreateWebCallParams,
    CreateWebCallResponse, ListCallsParams, Paginated, RegisterPhoneCallParams,
};
use reqwest::Method;
use serde_json::Value;

/// Call operations (`client.call()`).
#[derive(Clone)]
pub struct CallClient {
    pub(crate) http: HttpClient,
}

impl CallClient {
    /// `POST /v2/create-phone-call`
    pub async fn create_phone_call(&self, params: &CreatePhoneCallParams) -> Result<Call> {
        self.http
            .send_json(Method::POST, "/v2/create-phone-call", params)
            .await
    }

    /// `POST /v3/create-web-call`
    pub async fn create_web_call(
        &self,
        params: &CreateWebCallParams,
    ) -> Result<CreateWebCallResponse> {
        self.http
            .send_json(Method::POST, "/v3/create-web-call", params)
            .await
    }

    /// `POST /v2/register-phone-call`
    pub async fn register_phone_call(&self, params: &RegisterPhoneCallParams) -> Result<Call> {
        self.http
            .send_json(Method::POST, "/v2/register-phone-call", params)
            .await
    }

    /// `GET /v2/get-call/{call_id}`
    pub async fn get(&self, call_id: &str) -> Result<Call> {
        self.http
            .send_empty(Method::GET, &format!("/v2/get-call/{}", enc(call_id)))
            .await
    }

    /// `POST /v3/list-calls`
    pub async fn list(&self, params: &ListCallsParams) -> Result<Paginated<Call>> {
        self.http
            .send_json(Method::POST, "/v3/list-calls", params)
            .await
    }

    /// `DELETE /v2/delete-call/{call_id}`
    pub async fn delete(&self, call_id: &str) -> Result<()> {
        self.http
            .send_no_content(Method::DELETE, &format!("/v2/delete-call/{}", enc(call_id)))
            .await
    }

    /// `POST /v2/stop-call/{call_id}`
    pub async fn stop(&self, call_id: &str) -> Result<()> {
        self.http
            .send_no_content(Method::POST, &format!("/v2/stop-call/{}", enc(call_id)))
            .await
    }

    /// `PATCH /v2/update-call/{call_id}`
    pub async fn update(&self, call_id: &str, body: &Value) -> Result<Call> {
        self.http
            .send_json(
                Method::PATCH,
                &format!("/v2/update-call/{}", enc(call_id)),
                body,
            )
            .await
    }

    /// `PATCH /v2/update-live-call/{call_id}`
    pub async fn update_live(&self, call_id: &str, body: &Value) -> Result<Value> {
        self.http
            .send_json(
                Method::PATCH,
                &format!("/v2/update-live-call/{}", enc(call_id)),
                body,
            )
            .await
    }

    /// `POST /v2/listen-live-call/{call_id}`
    pub async fn listen_live(&self, call_id: &str) -> Result<Value> {
        self.http
            .send_empty(
                Method::POST,
                &format!("/v2/listen-live-call/{}", enc(call_id)),
            )
            .await
    }

    /// `POST /v2/take-over-live-call/{call_id}`
    pub async fn take_over_live(&self, call_id: &str, body: &Value) -> Result<Value> {
        self.http
            .send_json(
                Method::POST,
                &format!("/v2/take-over-live-call/{}", enc(call_id)),
                body,
            )
            .await
    }

    /// `PUT /rerun-call-analysis/{call_id}`
    pub async fn rerun_analysis(&self, call_id: &str) -> Result<()> {
        self.http
            .send_no_content(
                Method::PUT,
                &format!("/rerun-call-analysis/{}", enc(call_id)),
            )
            .await
    }

    /// `POST /create-batch-call`
    pub async fn create_batch(&self, params: &CreateBatchCallParams) -> Result<BatchCall> {
        self.http
            .send_json(Method::POST, "/create-batch-call", params)
            .await
    }
}
