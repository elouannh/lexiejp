use std::env::var;
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
	client: Client,
	ctx: Context<'_>,
	renshuu_api_key: String,
) -> Result<bool, Box<dyn Error>>
{
	let collection: Collection<user_struct::User> = client.database("lexie").collection("user");

	let filter: Document = doc! { "discord_id": ctx.author().id };
	let found_user: Option<user_struct::User> = collection.find_one(filter).await?;

	if found_user.is_some()
	{
		return Err(false.into())
	}

	let user = user_struct::User {
		discord_id: String::from(ctx.author().id),
		renshuu_api_key
	};

	let _saving: InsertOneResult = collection.insert_one(user).await.unwrap();
	Ok(true)
}

pub async fn delete_user(
	client: Client,
	ctx: Context<'_>,
) -> Result<bool, Box<dyn Error>>
{
	let collection: Collection<user_struct::User> = client.database("lexie").collection("user");

	let filter: Document = doc! { "discord_id": ctx.author().id };
	let found_user: Option<user_struct::User> = collection.find_one(filter).await?;

	if found_user.is_none()
	{
		return Err(false.into())
	}

	let _deleting: DeleteResult = collection.delete_one(filter.to_owned()).await.unwrap();
	Ok(true)
}