use crate::renshuu::rest_agent;

pub fn test_string(content: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(content).is_ok()
}

pub async fn test_token(token: &String) -> bool {
    const API_URL: &str = "https://api.renshuu.org/v1/profile";
    let rest_agent: rest_agent::RestAgent = rest_agent::RestAgent::new(token);

    match rest_agent
        .get_method(API_URL)
        .await
    {
        Ok(content) => test_string(&content),
        Err(_) => false,
    }
}
