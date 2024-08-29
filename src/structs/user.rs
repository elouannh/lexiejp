#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum UserPrivacyVisibility
{
	On,
	Off
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserPrivacy
{
	pub visibility: UserPrivacyVisibility
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User
{
	pub discord_id: String,
	pub renshuu_api_key: String,
	pub privacy: UserPrivacy
}
