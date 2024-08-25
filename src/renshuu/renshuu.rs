use std::error::Error;
use crate::renshuu::rest_agent::RestAgent;

#[allow(dead_code)]
pub async fn get_profile(token: &String) -> Result<String, Box<dyn Error>>
{
	let rest_agent: RestAgent = RestAgent::new(token);

	rest_agent.get_method("").await
}

pub fn test_string(rest_agent: &RestAgent, content: &str) -> bool
{
	match rest_agent.parse_json(&content)
	{
		Ok(_) =>
			{
				true
			}
		Err(_) =>
			{
				false
			}
	}
}

pub async fn test_token(token: &String) -> bool
{
	let rest_agent: RestAgent = RestAgent::new(token);

	match rest_agent.get_method("https://api.renshuu.org/v1/profile").await
	{
		Ok(content) =>
		{
			test_string(&rest_agent, &content)
		}
		Err(_) => {
			false
		}
	}
}