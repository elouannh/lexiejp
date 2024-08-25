mod db;
mod commands;
mod discord;
mod renshuu;
mod structs;
mod types;

#[poise_macros::command(slash_command)]
pub async fn profile(
	ctx: types::ctx::Context<'_>,
	#[description = "Selected user"] user: Option<serenity::all::User>,
) -> Result<(), types::ctx::Error>
{
	commands::profile::profile_cmd(&ctx, &user).await
}

#[poise_macros::command(slash_command)]
pub async fn register(
	ctx: types::ctx::Context<'_>,
	#[description = "Your Renshuu API key"] renshuu_api_key: String,
) -> Result<(), types::ctx::Error>
{
	commands::register::register_cmd(&ctx, &renshuu_api_key).await
}

#[tokio::main]
async fn main() {
	env_logger::init();
	let uri: String = std::env::var("MONGODB_URI").expect("missing MONGODB_URI");
	let client: mongodb::Client = mongodb::Client::with_uri_str(uri).await.unwrap();

	let token: String = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
	let intents: serenity::all::GatewayIntents = serenity::all::GatewayIntents::non_privileged();

	let framework: poise::Framework<types::ctx::Data, types::ctx::Error> =
		poise::Framework::builder()
			.options(poise::FrameworkOptions {
				commands: vec![profile(), register()],
				..Default::default()
			})
			.setup(|ctx, _ready, framework| {
				Box::pin(async move {
					poise::builtins::register_globally(ctx, &framework.options().commands).await?;
					Ok(types::ctx::Data {
						mongo_client: client,
					})
				})
			})
			.build();

	let client: serenity::all::Result<serenity::client::Client> =
		serenity::all::ClientBuilder::new(token, intents)
			.framework(framework)
			.await;
	client.unwrap().start().await.unwrap();
}