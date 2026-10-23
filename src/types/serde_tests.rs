//! Serialization unit tests (compiled as part of the library crate tests).

#[cfg(test)]
mod tests {
    use crate::types::{CreateAgentParams, ResponseEngine};
    use serde_json::json;

    #[test]
    fn response_engine_retell_llm_serde() {
        let engine = ResponseEngine::retell_llm("llm_xyz");
        let v = serde_json::to_value(&engine).unwrap();
        assert_eq!(
            v,
            json!({ "type": "retell-llm", "llm_id": "llm_xyz" })
        );
    }

    #[test]
    fn create_agent_flattens_extra() {
        let params = CreateAgentParams::new(ResponseEngine::retell_llm("llm_1"), "retell-Cimo")
            .agent_name("Bot")
            .extra("voice_temperature", json!(1.0));
        let v = serde_json::to_value(&params).unwrap();
        assert_eq!(v["agent_name"], "Bot");
        assert_eq!(v["voice_temperature"], 1.0);
        assert_eq!(v["voice_id"], "retell-Cimo");
    }
}
