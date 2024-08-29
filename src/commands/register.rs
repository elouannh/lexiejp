use crate::{db, renshuu, structs, types};

pub async fn register_cmd(
	ctx: &types::ctx::Context<'_>,
	renshuu_api_key: &String,
) -> Result<(), types::ctx::Error>
{
	let collection: mongodb::Collection<structs::user::User> =
		ctx.data().mongo_client.database("lexie").collection("user");

	let discord_id: String = String::from(ctx.author().id.to_string());
	let filter: mongodb::bson::Document = mongodb::bson::doc! { "discord_id": discord_id };
	let found_user: Option<structs::user::User> = collection.find_one(filter).await.unwrap();
	let is_existing: bool = found_user.is_some();
	let renshuu_clone: String = renshuu_api_key.clone().to_string();
	let tested: bool = renshuu::renshuu::test_token(&renshuu_clone).await;

	if !tested
	{
		let reply: poise::CreateReply = {
			let content: &str = "Invalid token provided.";

			poise::CreateReply::default()
				.content(content)
				.ephemeral(true)
		};
		ctx.send(reply).await?;
		return Ok(())
	}

	if is_existing
	{
		let reply: poise::CreateReply = {
			let content: &str = "API token edited successfully.";

			poise::CreateReply::default()
				.content(content)
				.ephemeral(true)
		};
		ctx.send(reply).await?;
	}
	else
	{
		let reply: poise::CreateReply = {
			let content: &str = "User successfully registered.";

			poise::CreateReply::default()
				.content(content)
				.ephemeral(true)
		};
		ctx.send(reply).await?;
	}

	let mongo_client: &mongodb::Client = &ctx.data().mongo_client;
	let valid_renshuu_api_key: String = String::from(&renshuu_clone);
	let _saving = db::user_service::register_user(mongo_client, ctx, &valid_renshuu_api_key).await;

	Ok(())
}