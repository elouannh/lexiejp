use mongodb::Client;
use poise::Context as PoiseContext;
use std::error::Error as StdError;

pub struct Data
{
	pub mongo_client: Client
}

pub type Error = Box<dyn StdError + Send + Sync>;

pub type Context<'a> = PoiseContext<'a, Data, Error>;
