use mongodb::
{
	bson::
	{
		doc,
		Document
	},
	results::InsertOneResult,
	Client,
	Collection
};
use std::error::Error;
use mongodb::results::DeleteResult;
use crate::structs::user as user_struct;
use crate::types::ctx::Context;

pub async fn register_user(
	client: &Client,
	ctx: &Context<'_>,
	renshuu_api_key: &String,
) -> Result<bool, Box<dyn Error>>
{
	let collection: Collection<user_struct::User> = client.database("lexie").collection("user");

	let discord_id: String = String::from(ctx.author().id.to_string());
	let filter: Document = doc! { "discord_id": discord_id };
	let found_user: Option<user_struct::User> = collection.find_one(filter).await?;

	if found_user.is_some()
	{
		return Ok(false);
	}

	let discord_id: String = String::from(ctx.author().id.to_string());
	let user = user_struct::User {
		discord_id,
		renshuu_api_key: renshuu_api_key.to_string()
	};

	let _saving: InsertOneResult = collection.insert_one(user).await.unwrap();
	Ok(true)
}

#[allow(dead_code)]
pub async fn delete_user(
	client: Client,
	ctx: Context<'_>,
) -> Result<bool, Box<dyn Error>>
{
	let collection: Collection<user_struct::User> = client.database("lexie").collection("user");

	let discord_id: String = String::from(ctx.author().id.to_string());
	let filter: Document = doc! { "discord_id": discord_id };
	let found_user: Option<user_struct::User> = collection.find_one(filter).await?;

	if found_user.is_none()
	{
		return Ok(false);
	}

	let doc_discord_id: String = String::from(ctx.author().id.to_string());
	let doc: Document = doc! { "discord_id": doc_discord_id };
	let _deleting: DeleteResult = collection.delete_one(doc).await.unwrap();
	Ok(true)
}