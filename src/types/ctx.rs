use crate::
{
	structs,
	types
};

type Context<'a> = poise::Context<'a, structs::ctx_data::CtxData, types::ctx_error>;
