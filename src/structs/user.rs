use serde::
{
	Deserialize,
	Serialize
};

#[derive(Debug, Serialize, Deserialize)]
pub struct User
{
	pub discord_id: String,
	pub renshuu_api_key: String,
}
