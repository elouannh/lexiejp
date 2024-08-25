use std::error::Error;
use crate::types::ctx::Context;

pub async fn reply(ctx: &Context<'_>, text: &str) -> Result<(), Box<dyn Error>>
{
	if let Err(why) = ctx.reply(text).await
	{
		return Ok(eprintln!("Message replying error: {why:?}"))
	}
	Ok(())
}