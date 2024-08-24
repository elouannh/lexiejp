use std::error::Error;

use crate::renshuu::rest_agent::RestAgent;

pub async fn get_profile(token: String) -> Result<String, Box<dyn Error>>
{
	let rest_agent: RestAgent = RestAgent::new(token);

	rest_agent.get_method("")
}