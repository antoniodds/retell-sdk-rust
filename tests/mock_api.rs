//! HTTP-level tests against a wiremock server (no real Retell account needed).

use retell_sdk::types::{
    CreateAgentParams, CreatePhoneCallParams, CreateWebCallParams, ListCallsParams,
    ResponseEngine, UpdateAgentParams,
};
use retell_sdk::RetellClient;
use serde_json::json;
use wiremock::matchers::{body_partial_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn client(server: &MockServer) -> RetellClient {
    RetellClient::with_base_url("test_key", server.uri())
}

#[tokio::test]
async fn list_voices_ok() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/list-voices"))
        .and(header("Authorization", "Bearer test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "voice_id": "retell-Cimo",
                "voice_name": "Cimo",
                "provider": "platform",
                "gender": "male",
                "accent": "American"
            }
        ])))
        .mount(&server)
        .await;

    let voices = client(&server).await.voice().list().await.unwrap();
    assert_eq!(voices.len(), 1);
    assert_eq!(voices[0].voice_id, "retell-Cimo");
    assert_eq!(voices[0].accent.as_deref(), Some("American"));
}

#[tokio::test]
async fn get_concurrency_ok() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/get-concurrency"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "current_concurrency": 2,
            "concurrency_limit": 20,
            "base_concurrency": 20,
            "purchased_concurrency": 0
        })))
        .mount(&server)
        .await;

    let c = client(&server).await.concurrency().get().await.unwrap();
    assert_eq!(c.current_concurrency, 2);
    assert_eq!(c.concurrency_limit, 20);
}

#[tokio::test]
async fn create_and_get_agent() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/create-agent"))
        .and(body_partial_json(json!({
            "voice_id": "retell-Cimo",
            "agent_name": "Jarvis"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "agent_id": "agent_abc",
            "version": 0,
            "agent_name": "Jarvis",
            "voice_id": "retell-Cimo",
            "response_engine": { "type": "retell-llm", "llm_id": "llm_1" },
            "last_modification_timestamp": 1
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/get-agent/agent_abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "agent_id": "agent_abc",
            "version": 0,
            "agent_name": "Jarvis",
            "voice_id": "retell-Cimo",
            "response_engine": { "type": "retell-llm", "llm_id": "llm_1" },
            "last_modification_timestamp": 1
        })))
        .mount(&server)
        .await;

    let c = client(&server).await;
    let params = CreateAgentParams::new(ResponseEngine::retell_llm("llm_1"), "retell-Cimo")
        .agent_name("Jarvis");
    let created = c.agent().create(&params).await.unwrap();
    assert_eq!(created.agent_id, "agent_abc");

    let fetched = c.agent().get("agent_abc").await.unwrap();
    assert_eq!(fetched.agent_name.as_deref(), Some("Jarvis"));
}

#[tokio::test]
async fn list_agents_paginated() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2/list-agents"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "has_more": false,
            "items": [{
                "agent_id": "agent_1",
                "is_suspended": false,
                "agent_name": "Support",
                "channel": "voice",
                "user_modified_timestamp": 100,
                "tags": {}
            }]
        })))
        .mount(&server)
        .await;

    let page = client(&server).await.agent().list_all().await.unwrap();
    assert!(!page.has_more);
    assert_eq!(page.items[0].agent_name, "Support");
}

#[tokio::test]
async fn update_agent() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/update-agent/agent_1"))
        .and(body_partial_json(json!({ "agent_name": "Renamed" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "agent_id": "agent_1",
            "version": 1,
            "agent_name": "Renamed",
            "voice_id": "retell-Cimo",
            "last_modification_timestamp": 2
        })))
        .mount(&server)
        .await;

    let updated = client(&server)
        .await
        .agent()
        .update(
            "agent_1",
            &UpdateAgentParams {
                agent_name: Some("Renamed".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.agent_name.as_deref(), Some("Renamed"));
}

#[tokio::test]
async fn delete_agent_no_content() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/delete-agent/agent_1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    client(&server)
        .await
        .agent()
        .delete("agent_1")
        .await
        .unwrap();
}

#[tokio::test]
async fn create_phone_call() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2/create-phone-call"))
        .and(body_partial_json(json!({
            "from_number": "+14157774444",
            "to_number": "+12137774445"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "call_id": "call_1",
            "agent_id": "agent_1",
            "agent_version": 0,
            "call_status": "registered",
            "call_type": "phone_call",
            "from_number": "+14157774444",
            "to_number": "+12137774445",
            "direction": "outbound"
        })))
        .mount(&server)
        .await;

    let call = client(&server)
        .await
        .call()
        .create_phone_call(&CreatePhoneCallParams::new(
            "+14157774444",
            "+12137774445",
        ))
        .await
        .unwrap();
    assert_eq!(call.call_id, "call_1");
    assert_eq!(call.from_number.as_deref(), Some("+14157774444"));
}

#[tokio::test]
async fn create_web_call() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v3/create-web-call"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "call_id": "call_web",
            "access_token": "tok_abc",
            "transport": "gateway",
            "ice_servers": [{ "urls": "stun:stun.l.google.com:19302" }],
            "expires_at": 1735689600000_i64
        })))
        .mount(&server)
        .await;

    let resp = client(&server)
        .await
        .call()
        .create_web_call(&CreateWebCallParams::new("agent_1"))
        .await
        .unwrap();
    assert_eq!(resp.access_token, "tok_abc");
    assert_eq!(resp.ice_servers.len(), 1);
}

#[tokio::test]
async fn list_calls() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v3/list-calls"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "has_more": false,
            "items": [{
                "call_id": "call_1",
                "agent_id": "agent_1",
                "agent_version": 0,
                "call_status": "ended",
                "call_type": "web_call"
            }]
        })))
        .mount(&server)
        .await;

    let page = client(&server)
        .await
        .call()
        .list(&ListCallsParams::default())
        .await
        .unwrap();
    assert_eq!(page.items.len(), 1);
}

#[tokio::test]
async fn phone_number_path_keeps_plus() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/get-phone-number/+14157774444"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "phone_number": "+14157774444",
            "phone_number_type": "retell-twilio",
            "last_modification_timestamp": 1,
            "nickname": "Front desk"
        })))
        .mount(&server)
        .await;

    let pn = client(&server)
        .await
        .phone_number()
        .get("+14157774444")
        .await
        .unwrap();
    assert_eq!(pn.nickname.as_deref(), Some("Front desk"));
}

#[tokio::test]
async fn api_error_unauthorized() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/list-voices"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "status": "error",
            "message": "API key is missing or invalid."
        })))
        .mount(&server)
        .await;

    let err = client(&server).await.voice().list().await.unwrap_err();
    assert!(err.is_unauthorized());
    assert!(err.to_string().contains("API key"));
}

#[tokio::test]
async fn api_error_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/get-agent/missing"))
        .respond_with(ResponseTemplate::new(422).set_body_json(json!({
            "status": "error",
            "message": "Cannot find requested asset under given api key."
        })))
        .mount(&server)
        .await;

    let err = client(&server).await.agent().get("missing").await.unwrap_err();
    assert_eq!(err.status().map(|s| s.as_u16()), Some(422));
}

#[tokio::test]
async fn create_llm_and_kb() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/create-retell-llm"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "llm_id": "llm_1",
            "version": 0,
            "general_prompt": "You are helpful.",
            "last_modification_timestamp": 1
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/create-knowledge-base"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "knowledge_base_id": "kb_1",
            "knowledge_base_name": "FAQ",
            "status": "in_progress"
        })))
        .mount(&server)
        .await;

    let c = client(&server).await;
    let llm = c
        .llm()
        .create(
            &retell_sdk::types::CreateRetellLlmParams::new()
                .general_prompt("You are helpful."),
        )
        .await
        .unwrap();
    assert_eq!(llm.llm_id, "llm_1");

    let kb = c
        .knowledge_base()
        .create(&retell_sdk::types::CreateKnowledgeBaseParams::new("FAQ"))
        .await
        .unwrap();
    assert_eq!(kb.knowledge_base_id, "kb_1");
}
