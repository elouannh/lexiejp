use reqwest::
{
	Client,
	Response
};
use std::error::Error;

pub struct RestAgent
{
	pub token: String,
	pub client: Client,
}

impl RestAgent
{
	pub fn new(token: String) -> Self
	{
		RestAgent {
			token,
			client: Client::new(),
		}
	}

	pub async fn get_method(&self, url: &str) -> Result<String, Box<dyn Error>>
	{
		let response: Response = self.client
			.get(url)
			.bearer_auth(&self.token)
			.send()
			.await?;

		if response.status().is_success()
		{
			let body = response.text().await?;
			Ok(body)
		}
		else
		{
			Err(format!("Failed to get resources: {}", response.status()).into())
		}
	}
}