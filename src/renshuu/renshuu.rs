use std::error::Error;
use serde_json::Value;
use crate::renshuu::rest_agent::RestAgent;

pub async fn get_profile(token: String) -> Result<String, Box<dyn Error>>
{
	let rest_agent: RestAgent = RestAgent::new(token);

	rest_agent.get_method("").await
}

pub async fn test_token(token: String) -> Result<bool, Box<dyn Error>>
{
	let rest_agent: RestAgent = RestAgent::new(token);

	let response: String = rest_agent.get_method("/profile").await?;
	let json: Value = rest_agent.parse_json(**response)?;

	if json.get("error").is_some()
	{
		return Ok(false)
	}
	Ok(true)
}