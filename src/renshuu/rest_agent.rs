use crate::types::ctx;

pub struct RestAgent {
    pub token: String,
    pub client: reqwest::Client,
}

impl RestAgent {
    pub fn new(token: &String) -> Self {
        RestAgent {
            token: token.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_method(&self, route: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response: reqwest::Response = self
            .client
            .get(route)
            .bearer_auth(&self.token)
            .send()
            .await?;

        if response.status().is_success() {
            let body: String = response.text().await?;
            Ok(body)
        } else {
            Err(format!("Failed to get resources: {}", response.status()).into())
        }
    }

    pub fn parse_json(&self, user_str: &str) -> Result<serde_json::Value, ctx::Error> {
        let json: serde_json::Value = serde_json::from_str(user_str).unwrap();
        Ok(json)
    }
}
