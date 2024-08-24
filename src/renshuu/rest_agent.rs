use reqwest::
{
	Client,
	Response
};
use serde_json::Value;
use std::error::Error;

pub struct RestAgent
{
	pub token: String,
	pub client: Client,
	pub endpoint: String
}

impl RestAgent
{
	pub fn new(token: String) -> Self
	{
		RestAgent {
			token,
			client: Client::new(),
			endpoint: String::from("https://api.renshuu.org/v1")
		}
	}

	pub fn get_full_url(&self, route: &str) -> String
	{
		let mut endpoint_radical: String = self.endpoint.to_owned();
		let route_suffix: String = String::from(route).to_owned();

		endpoint_radical.push_str(&route_suffix);
		route_suffix
	}

	pub async fn get_method(&self, route: &str) -> Result<String, Box<dyn Error>>
	{
		let response: Response = self.client
			.get(self.get_full_url(route))
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

	pub async fn post_method(&self, route: &str, data: &Value) -> Result<String, Box<dyn Error>>
	{
		let response: Response = self.client
			.post(self.get_full_url(route))
			.bearer_auth(&self.token)
			.json(data)
			.send()
			.await?;

		if response.status().is_success()
		{
			let body: String = response.text().await?;
			Ok(body)
		}
		else
		{
			Err(format!("Failed to post data: {}", response.status()).into())
		}
	}
}