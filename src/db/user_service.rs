use crate::
{
	structs,
	types
};

pub async fn register_user(
	client: &mongodb::Client,
	ctx: &types::ctx::Context<'_>,
	renshuu_api_key: &String,
) -> Result<bool, Box<dyn std::error::Error>>
{
	let collection: mongodb::Collection<structs::user::User> =
		client.database("lexie").collection("user");

	let discord_id: String = String::from(ctx.author().id.to_string());
	let filter: mongodb::bson::Document = mongodb::bson::doc! { "discord_id": discord_id };
	let found_user: Option<structs::user::User> = collection.find_one(filter).await?;

	if found_user.is_some()
	{
		return Ok(false);
	}

	let discord_id: String = String::from(ctx.author().id.to_string());
	let user = structs::user::User {
		discord_id,
		renshuu_api_key: renshuu_api_key.to_string(),
		privacy: structs::user::UserPrivacy {
			visibility: structs::user::UserPrivacyVisibility::Off
		}
	};

	let _saving: mongodb::results::InsertOneResult = collection.insert_one(user).await.unwrap();
	Ok(true)
}