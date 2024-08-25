use mongodb::
{
	bson::
	{
		doc,
		Document
  	},
	Client,
	Collection
};

use crate::db::user_service::register_user;
use crate::discord::reply as discord_reply;
use crate::renshuu::renshuu::test_token;
use crate::structs::user::User as UserStruct;
use crate::types::ctx::
{
	Context,
	Error as CtxError
};

pub async fn register_cmd(
	ctx: Context<'_>,
	renshuu_api_key: Option<String>,
) -> Result<(), CtxError>
{
	let collection: Collection<UserStruct> = ctx.data().mongo_client.database("lexie").collection("user");

	let discord_id: String = String::from(ctx.author().id.to_string());
	let filter: Document = doc! { "discord_id": discord_id };
	let found_user: Option<UserStruct> = collection.find_one(filter).await.unwrap();
	let is_existing: bool = found_user.is_some();

	match test_token(renshuu_api_key.unwrap()).await {
		Ok(false) => {
			return discord_reply::reply(**ctx, "Invalid token provided.")?
		}
		Ok(true) => {
			if is_existing
			{
				discord_reply::reply(**ctx, "API token edited successfully.")?
			}
			else
			{
				discord_reply::reply(**ctx, "User successfully registered.")?
			}
		}
		Err(e) => {
			eprintln!("Error: {}", e);
			return discord_reply::reply(**ctx, &format!("```An error occurred: {}```", e))?
		}
	}

	let mongo_client: &Client = &ctx.data().mongo_client;
	let valid_renshuu_api_key: String = String::from(renshuu_api_key.to_owned());
	let _saving = register_user(mongo_client, ctx, valid_renshuu_api_key).await;
	Ok(())
}