use poise::{
	serenity_prelude as serenity,
	Framework
};
use serenity::all::{
	GatewayIntents,
	User
};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn age(
	ctx: Context<'_>,
	#[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
	let u: &User = user.as_ref().unwrap_or_else(|| ctx.author());
	let response: String = format!("{}'s account was created at {}", u.name, u.created_at());
	ctx.say(response).await?;
	Ok(())
}

#[tokio::main]
async fn main() {
	let token: String = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
	let intents: GatewayIntents = serenity::GatewayIntents::non_privileged();

	let framework: Framework<Data, Error> = Framework::builder()
		.options(poise::FrameworkOptions {
			commands: vec![age()],
			..Default::default()
		})
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				Ok(Data {})
			})
		})
		.build();

	let client = serenity::ClientBuilder::new(token, intents)
		.framework(framework)
		.await;
	client.unwrap().start().await.unwrap();
	println!("TEST\n")
}