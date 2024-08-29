use crate::types;

pub async fn user_does_not_exist(ctx: &types::ctx::Context<'_>, ephemeral: bool) -> Result<(), types::ctx::Error>
{
	let reply: poise::CreateReply = {
		let content: &str = "User does not exist. Do /register before.";

		poise::CreateReply::default()
			.content(content)
			.ephemeral(ephemeral)
	};
	ctx.send(reply).await?;
	Ok(())
}