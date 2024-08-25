use mongodb::
{
	bson::
	{
		doc,
		Document
  	},
	Client,
	Collection
};
use crate::db::user_service::register_user;
use crate::renshuu::renshuu::test_token;
use crate::renshuu::rest_agent::RestAgent;
use crate::structs::user::User as UserStruct;
use crate::types::ctx::
{
	Context,
	Error as CtxError
};

pub async fn register_cmd(
	ctx: &Context<'_>,
	renshuu_api_key: &String,
) -> Result<(), CtxError>
{
	let collection: Collection<UserStruct> = ctx.data().mongo_client.database("lexie").collection("user");

	let discord_id: String = String::from(ctx.author().id.to_string());
	let filter: Document = doc! { "discord_id": discord_id };
	let found_user: Option<UserStruct> = collection.find_one(filter).await.unwrap();
	let is_existing: bool = found_user.is_some();
	let renshuu_clone: String = renshuu_api_key.clone().to_string();
	let tested: bool = test_token(&renshuu_clone).await;

	if !tested
	{
		ctx.reply("Invalid token provided").await?;
		return Ok(())
	}

	if is_existing
	{
		ctx.reply("API token edited successfully.").await?;
	} else {
		ctx.reply("User successfully registered.").await?;
	}

	let mongo_client: &Client = &ctx.data().mongo_client;
	let valid_renshuu_api_key: String = String::from(&renshuu_clone);
	let _saving = register_user(mongo_client, ctx, &valid_renshuu_api_key).await;

	Ok(())
}