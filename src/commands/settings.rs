use crate::{db, discord, renshuu, replies, structs, types};

pub async fn settings_cmd(ctx: &types::ctx::Context<'_>) -> Result<(), types::ctx::Error> {
    let collection: mongodb::Collection<structs::user::User> =
        db::access::get_collection(&ctx.data().mongo_client);
    let discord_id: String = String::from(ctx.author().id.to_string());
    let filter: mongodb::bson::Document = mongodb::bson::doc! { "discord_id": discord_id };
    let found_user: Option<structs::user::User> = collection.find_one(filter).await.unwrap();
    let is_existing: bool = found_user.is_some();

    if !is_existing {
        replies::default_replies::user_does_not_exist(ctx, false).await?;
        return Ok(());
    }

    let rest_agent: renshuu::rest_agent::RestAgent =
        renshuu::rest_agent::RestAgent::new(&found_user.unwrap().renshuu_api_key);
    let content: &String = &rest_agent
        .get_method("https://api.renshuu.org/v1/profile")
        .await
        .expect("Something went wrong with the API.");
    let _reply: poise::CreateReply = {
        let embed: poise::serenity_prelude::CreateEmbed = discord::embeds::schedule_embed(content);

        poise::CreateReply::default().embed(embed)
    };
    // ctx.send(reply).await.unwrap();
    Ok(())
}
