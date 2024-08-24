mod commands;
use commands::profile::profile_cmd;
mod renshuu;
mod structs;
mod types;
use types::ctx::
{
	Context,
	Data as CtxData,
	Error as CtxError
};

use mongodb::Client as MongoClient;
use poise::
{
	builtins::register_globally,
	serenity_prelude as serenity,
	Framework,
	FrameworkOptions
};
use poise_macros::command;
use serenity::all::
{
	ClientBuilder,
	GatewayIntents,
	User
};
use std::
{
	env::var,
	error::Error as StdError
};

#[command(slash_command)]
pub async fn profile(
	ctx: Context<'_>,
	#[description = "Selected user"] user: Option<User>,
) -> Result<(), CtxError> {
	profile_cmd(ctx, user)
}

#[tokio::main]
async fn main() {
	let uri: String = var("DISCORD_TOKEN").expect("missing MONGODB_URI");
	let client: MongoClient = MongoClient::with_uri_str(uri).await.unwrap();

	let token: String = var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
	let intents: GatewayIntents = GatewayIntents::non_privileged();

	let framework: Framework<CtxData, CtxError> = Framework::builder()
		.options(FrameworkOptions {
			commands: vec![profile()],
			..Default::default()
		})
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				register_globally(ctx, &framework.options().commands).await?;
				Ok(CtxData {
					mongo_client: client,
				})
			})
		})
		.build();

	let client = ClientBuilder::new(token, intents)
		.framework(framework)
		.await;
	client.unwrap().start().await.unwrap();
	println!("TEST\n")
}