use mongodb::bson::{doc, Document};
use mongodb::Collection;
use serenity::all::User;
use crate::renshuu::rest_agent::RestAgent;
use crate::types::ctx::
{
	Context,
	Error as CtxError
};

pub async fn profile_cmd(
	ctx: &Context<'_>,
	user: &Option<User>,
) -> Result<(), CtxError>
{
	let collection: Collection<crate::structs::user::User> = ctx.data().mongo_client.database("lexie").collection("user");

	let discord_id: String = String::from(ctx.author().id.to_string());
	let filter: Document = doc! { "discord_id": discord_id };
	let found_user: Option<crate::structs::user::User> = collection.find_one(filter).await.unwrap();
	let is_existing: bool = found_user.is_some();

	if !is_existing
	{
		ctx.reply("User does not exist. Do /register before.").await?;
		return Ok(())
	}
	let rest_agent: RestAgent = RestAgent::new(&found_user.unwrap().renshuu_api_key);

	ctx.reply(format!("{}", rest_agent.get_method("https://api.renshuu.org/v1/profile").await.unwrap())).await?;
	Ok(())
}