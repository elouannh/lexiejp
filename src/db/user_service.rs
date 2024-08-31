use crate::classes::renshuu_user;
use crate::db::access;
use crate::structs::user;
use crate::types::ctx;

pub async fn user_exists(ctx: &ctx::Context<'_>) -> bool {
    let coll: mongodb::Collection<user::User> =
        access::get_collection(&ctx.data().mongo_client);
    let discord_id: String = String::from(ctx.author().id.to_string());
    let filter: mongodb::bson::Document = mongodb::bson::doc! { "discord_id": discord_id };
    let found_user: Option<user::User> = coll.find_one(filter).await.unwrap();

    if found_user.is_some() {
        return true;
    }

    false
}

pub async fn get_user(
    ctx: &ctx::Context<'_>,
) -> Result<user::User, Box<dyn std::error::Error>> {
    let coll: mongodb::Collection<user::User> =
        access::get_collection(&ctx.data().mongo_client);
    let discord_id: String = String::from(ctx.author().id.to_string());
    let filter: mongodb::bson::Document = mongodb::bson::doc! { "discord_id": discord_id };
    let found_user: Option<user::User> = coll.find_one(filter).await?;

    Ok(found_user.expect("User does not exist."))
}

pub async fn register_user(
    ctx: &ctx::Context<'_>,
    renshuu_api_key: &String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let coll: mongodb::Collection<user::User> = access::get_collection(&ctx.data().mongo_client);
    let renshuu_user: renshuu_user::RenshuuUser =
        renshuu_user::RenshuuUser::new(ctx, renshuu_api_key).await;
    let _saving: mongodb::results::InsertOneResult =
        coll.insert_one(renshuu_user.register_data()).await.unwrap();
    Ok(true)
}

pub async fn delete_user(
    client: &mongodb::Client,
    ctx: &ctx::Context<'_>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let collection: mongodb::Collection<user::User> = access::get_collection(client);
    let discord_id: String = String::from(ctx.author().id.to_string());
    let filter: mongodb::bson::Document = mongodb::bson::doc! { "discord_id": discord_id };
    let _deleting = collection.find_one_and_delete(filter);
    Ok(true)
}
