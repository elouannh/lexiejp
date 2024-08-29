use crate::{classes, discord, renshuu, replies, types};

fn set_default(ctx: &types::ctx::Context<'_>) -> serenity::all::User
{
	ctx.author().clone()
}

pub async fn profile_cmd(
	ctx: &types::ctx::Context<'_>,
	user: &Option<serenity::all::User>,
) -> Result<(), types::ctx::Error>
{
	let user_profile: &serenity::all::User = &user.clone().unwrap_or_else(|| set_default(ctx));
	let collection: mongodb::Collection<crate::structs::user::User> =
		ctx.data().mongo_client.database("lexie").collection("user");

	let discord_id: String = String::from(user_profile.id.to_string());
	let filter: mongodb::bson::Document = mongodb::bson::doc! { "discord_id": discord_id };
	let found_user: Option<crate::structs::user::User> = collection.find_one(filter).await.unwrap();
	let is_existing: bool = found_user.is_some();

	if !is_existing
	{
		replies::default_replies::user_does_not_exist(ctx, false).await?;
		return Ok(())
	}
	let rest_agent: renshuu::rest_agent::RestAgent =
		renshuu::rest_agent::RestAgent::new(&found_user.unwrap().renshuu_api_key);

	let content: &String = &rest_agent.get_method("https://api.renshuu.org/v1/profile")
		.await.expect("Something went wrong with the API.");
	let renshuu_user: classes::renshuu_user::RenshuuUser = classes::renshuu_user::RenshuuUser::new(
		user_profile.to_owned(), content.to_owned()
	);

	let reply: poise::CreateReply = {
		let embed: poise::serenity_prelude::CreateEmbed =
			discord::embeds::profile_embed(&renshuu_user);

		poise::CreateReply::default()
			.embed(embed)
	};
	ctx.send(reply).await.unwrap();
	Ok(())
}