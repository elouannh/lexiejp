use reqwest::
{
	Client,
	Response
};
use serde_json::
{
	from_str,
	Value
};
use std::error::Error;
use crate::types::ctx::Context as CtxError;

pub struct RestAgent
{
	pub token: String,
	pub client: Client,
}

impl RestAgent
{
	pub fn new(token: &String) -> Self
	{
		RestAgent {
			token: token.to_string(),
			client: Client::new(),
		}
	}

	pub async fn get_method(&self, route: &str) -> Result<String, Box<dyn Error>>
	{
		let response: Response = self.client
			.get(route)
			.bearer_auth(&self.token)
			.send()
			.await?;

		if response.status().is_success()
		{
			let body: String = response.text().await?;
			Ok(body)
		}
		else
		{
			Err(format!("Failed to get resources: {}", response.status()).into())
		}
	}

	pub fn parse_json(&self, user_str: &str) -> Result<Value, CtxError>
	{
		let json: Value = from_str(user_str).unwrap();
		Ok(json)
	}
}