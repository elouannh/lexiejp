use crate::{replies, structs, types};

pub async fn leave_cmd(
	ctx: &types::ctx::Context<'_>,
) -> Result<(), types::ctx::Error>
{
	let collection: mongodb::Collection<structs::user::User> =
		ctx.data().mongo_client.database("lexie").collection("user");

	let discord_id: String = String::from(ctx.author().id.to_string());
	let filter: mongodb::bson::Document = mongodb::bson::doc! { "discord_id": discord_id };
	let found_user: Option<crate::structs::user::User> = collection.find_one(filter).await.unwrap();
	let is_existing: bool = found_user.is_some();

	if !is_existing
	{
		replies::default_replies::user_does_not_exist(ctx, true).await?;
		return Ok(())
	}

	let _deleted = collection.find_one_and_delete(
		mongodb::bson::doc! { "discord_id": String::from(ctx.author().id.to_string()) }
	);

	let reply: poise::CreateReply = {
		let content: &str = "You have been deleted successfully.";

		poise::CreateReply::default()
			.content(content)
			.ephemeral(true)
	};
	ctx.send(reply).await.unwrap();
	Ok(())
}