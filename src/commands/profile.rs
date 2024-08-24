use serenity::all::User;

use crate::types::ctx::
{
	Context,
	Error as CtxError
};

pub async fn profile_cmd(
	ctx: Context<'_>,
	user: Option<User>,
) -> Result<(), CtxError>
{
	let _ = ctx;
	let _2 = user;
	Ok(())
}