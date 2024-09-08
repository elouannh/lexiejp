use crate::classes::renshuu_user;
use crate::db::{access, user_service};
use crate::discord::embeds;
use crate::classes::rest_agent;
use crate::replies::default_replies;
use crate::structs::user;
use crate::types::ctx;

fn set_default(ctx: &ctx::Context<'_>) -> serenity::all::User {
    ctx.author().clone()
}

pub async fn profile_cmd(
    ctx: &ctx::Context<'_>,
    user: &Option<serenity::all::User>,
) -> Result<(), ctx::Error> {
    let is_existing: bool = user_service::user_exists(ctx).await;

    if !is_existing {
        default_replies::user_does_not_exist(ctx, false).await?;
        return Ok(());
    }

    let user_profile: &serenity::all::User = &user.clone().unwrap_or_else(|| set_default(ctx));
    let filter: mongodb::bson::Document =
        mongodb::bson::doc! { "discord_id": user_profile.id.to_string() };
    let coll: mongodb::Collection<user::User> = access::get_collection(&ctx.data().mongo_client);
    let user_data: Option<user::User> = coll.find_one(filter).await?;
    let rest_agent: rest_agent::RestAgent =
        rest_agent::RestAgent::new(&user_data.unwrap().renshuu_api_key);
    let renshuu_user: renshuu_user::RenshuuUser =
        renshuu_user::RenshuuUser::new(ctx, &rest_agent.token.to_string()).await;
    let reply: poise::CreateReply = {
        let embed: poise::serenity_prelude::CreateEmbed = embeds::profile_embed(&renshuu_user);

        poise::CreateReply::default().embed(embed)
    };
    ctx.send(reply).await.unwrap();
    Ok(())
}
