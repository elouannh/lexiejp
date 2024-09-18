use crate::db::access;
use crate::discord::embeds;
use crate::classes::rest_agent;
use crate::replies::default_replies;
use crate::structs::user;
use crate::types::ctx;

pub async fn settings_cmd(ctx: &ctx::Context<'_>) -> Result<(), ctx::Error> {
    let collection: mongodb::Collection<user::User> =
        access::get_collection(&ctx.data().mongo_client);
    let discord_id: String = String::from(ctx.author().id.to_string());
    let filter: mongodb::bson::Document = mongodb::bson::doc! { "discord_id": discord_id };
    let found_user: Option<user::User> = collection.find_one(filter).await.unwrap();
    let is_existing: bool = found_user.is_some();

    if !is_existing {
        default_replies::user_does_not_exist(ctx, false).await?;
        return Ok(());
    }

    let rest_agent: rest_agent::RestAgent =
        rest_agent::RestAgent::new(&found_user.unwrap().renshuu_api_key);
    let content: &String = &rest_agent
        .get_method("https://api.renshuu.org/v1/profile")
        .await
        .expect("Something went wrong with the API.");
    let _reply: poise::CreateReply = {
        let embed: poise::serenity_prelude::CreateEmbed = embeds::schedule_embed(content);

        poise::CreateReply::default().embed(embed)
    };
    // ctx.send(reply).await.unwrap();
    Ok(())
}
