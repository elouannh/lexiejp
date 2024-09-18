use crate::types::ctx;

pub async fn send_message(
    ctx: &ctx::Context<'_>,
    content: &str,
    ephemeral: bool,
) -> Result<(), ctx::Error> {
    let reply: poise::CreateReply = poise::CreateReply::default()
        .content(content)
        .ephemeral(ephemeral);
    ctx.send(reply).await?;
    Ok(())
}

pub async fn user_does_not_exist(
    ctx: &ctx::Context<'_>,
    ephemeral: bool,
) -> Result<(), ctx::Error> {
    send_message(ctx, "User does not exist. Do /register before", ephemeral).await
}

pub async fn user_deleted_successfully(
    ctx: &ctx::Context<'_>,
    ephemeral: bool,
) -> Result<(), ctx::Error> {
    send_message(ctx, "You have been deleted successfully.", ephemeral).await
}

pub async fn invalid_token_provided(
    ctx: &ctx::Context<'_>,
    ephemeral: bool,
) -> Result<(), ctx::Error> {
    send_message(ctx, "Invalid token provided.", ephemeral).await
}

pub async fn api_token_updated_successfully(
    ctx: &ctx::Context<'_>,
    ephemeral: bool,
) -> Result<(), ctx::Error> {
    send_message(ctx, "API token updated successfully.", ephemeral).await
}

pub async fn user_successfully_registered(
    ctx: &ctx::Context<'_>,
    ephemeral: bool,
) -> Result<(), ctx::Error> {
    send_message(ctx, "User successfully registered.", ephemeral).await
}
