use crate::renshuu;
use crate::types;

pub async fn profile_cmd(
	ctx: &types::ctx::Context<'_>,
	user: &Option<serenity::all::User>,
) -> Result<(), types::ctx::Error>
{
	let _void_user = user;
	let collection: mongodb::Collection<crate::structs::user::User> =
		ctx.data().mongo_client.database("lexie").collection("user");

	let discord_id: String = String::from(ctx.author().id.to_string());
	let filter: mongodb::bson::Document = mongodb::bson::doc! { "discord_id": discord_id };
	let found_user: Option<crate::structs::user::User> = collection.find_one(filter).await.unwrap();
	let is_existing: bool = found_user.is_some();

	if !is_existing
	{
		ctx.reply("User does not exist. Do /register before.").await?;
		return Ok(())
	}
	let rest_agent: renshuu::rest_agent::RestAgent =
		renshuu::rest_agent::RestAgent::new(&found_user.unwrap().renshuu_api_key);

	ctx.reply(
		format!(
			"{}", rest_agent.get_method("https://api.renshuu.org/v1/profile").await.unwrap()
		)
	).await?;
	Ok(())
}