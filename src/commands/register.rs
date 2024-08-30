use crate::{db, renshuu, replies, structs, types};

pub async fn register_cmd(
    ctx: &types::ctx::Context<'_>,
    renshuu_api_key: &String,
) -> Result<(), types::ctx::Error> {
    let collection: mongodb::Collection<structs::user::User> =
        db::access::get_collection(&ctx.data().mongo_client);
    let discord_id: String = String::from(ctx.author().id.to_string());
    let filter: mongodb::bson::Document = mongodb::bson::doc! { "discord_id": discord_id };
    let found_user: Option<structs::user::User> = collection.find_one(filter).await.unwrap();
    let is_existing: bool = found_user.is_some();
    let renshuu_clone: String = renshuu_api_key.clone().to_string();
    let tested: bool = renshuu::renshuu::test_token(&renshuu_clone).await;

    if !tested {
        replies::default_replies::invalid_token_provided(ctx, true).await?;
        return Ok(());
    }
    if is_existing {
        replies::default_replies::api_token_updated_successfully(ctx, true).await?;
    } else {
        replies::default_replies::user_successfully_registered(ctx, true).await?;
    }

    let valid_renshuu_api_key: String = String::from(&renshuu_clone);
    let _saving = db::user_service::register_user(ctx, &valid_renshuu_api_key).await;
    Ok(())
}
