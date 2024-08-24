use poise::Context as PoiseContext;
use serenity::all::User;
use std::error::Error as StdError;

use crate::structs::ctx_data as ctx_data_struct;

type Error = Box<dyn StdError + Send + Sync>;
type Context<'a> = PoiseContext<'a, ctx_data_struct::CtxData, Error>;

pub async fn profile_cmd(
	ctx: Context<'_>,
	user: Option<User>,
) -> Result<(), Error>
{
	let u: &User = user.as_ref().unwrap_or_else(|| ctx.author());
	let response: String = format!("{}'s account was created at {}", u.name, u.created_at());
	ctx.say(response).await?;
	Ok(())
}