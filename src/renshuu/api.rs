use crate::classes::rest_agent;

pub fn test_string(rest_agent: &rest_agent::RestAgent, content: &str) -> bool {
    match rest_agent.parse_json(&content) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn test_token(token: &String) -> bool {
    let rest_agent: rest_agent::RestAgent = rest_agent::RestAgent::new(token);

    match rest_agent
        .get_method("https://api.renshuu.org/v1/profile")
        .await
    {
        Ok(content) => test_string(&rest_agent, &content),
        Err(_) => false,
    }
}
