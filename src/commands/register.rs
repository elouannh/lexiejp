use mongodb::
{
	bson::
	{
		doc,
		Document
	},
	Collection
};
use serenity::all::User;

use crate::db::user_service::register_user;
use crate::renshuu::renshuu;
use crate::structs::user::User as UserStruct;
use crate::types::ctx::
{
	Context,
	Error as CtxError
};

pub async fn register_cmd(
	ctx: Context<'_>,
	renshuu_api_key: Option<String>,
) -> Result<(), CtxError>
{
	let collection: Collection<UserStruct> = ctx.data().mongo_client.database("lexie").collection("user");

	let filter: Document = doc! { "discord_id": ctx.author().id };
	let found_user: Option<UserStruct> = collection.find_one(filter).await.unwrap();

	if found_user.is_some()
	{
		if let Err(why) = ctx.channel_id().say(&ctx.http(), "User already exists.").await
		{
			return println!("Message sending error: {why:?}").into()
		}
		return Err(0.into());
	}

	let _saving = register_user(**ctx.data().mongo_client, ctx, renshuu_api_key.unwrap()).await;
	Ok(())
}