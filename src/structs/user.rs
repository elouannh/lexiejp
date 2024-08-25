#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User
{
	pub discord_id: String,
	pub renshuu_api_key: String,
}
