use crate::types::ctx;

pub async fn user_does_not_exist(
    ctx: &ctx::Context<'_>,
    ephemeral: bool,
) -> Result<(), ctx::Error> {
    let reply: poise::CreateReply = {
        let content: &str = "User does not exist. Do /register before.";

        poise::CreateReply::default()
            .content(content)
            .ephemeral(ephemeral)
    };
    ctx.send(reply).await?;
    Ok(())
}

pub async fn user_deleted_successfully(
    ctx: &ctx::Context<'_>,
    ephemeral: bool,
) -> Result<(), ctx::Error> {
    let reply: poise::CreateReply = {
        let content: &str = "You have been deleted successfully.";

        poise::CreateReply::default()
            .content(content)
            .ephemeral(ephemeral)
    };
    ctx.send(reply).await?;
    Ok(())
}

pub async fn invalid_token_provided(
    ctx: &ctx::Context<'_>,
    ephemeral: bool,
) -> Result<(), ctx::Error> {
    let reply: poise::CreateReply = {
        let content: &str = "Invalid token provided.";

        poise::CreateReply::default()
            .content(content)
            .ephemeral(ephemeral)
    };
    ctx.send(reply).await?;
    Ok(())
}

pub async fn api_token_updated_successfully(
    ctx: &ctx::Context<'_>,
    ephemeral: bool,
) -> Result<(), ctx::Error> {
    let reply: poise::CreateReply = {
        let content: &str = "API token updated successfully.";

        poise::CreateReply::default()
            .content(content)
            .ephemeral(ephemeral)
    };
    ctx.send(reply).await?;
    Ok(())
}

pub async fn user_successfully_registered(
    ctx: &ctx::Context<'_>,
    ephemeral: bool,
) -> Result<(), ctx::Error> {
    let reply: poise::CreateReply = {
        let content: &str = "User successfully registered.";

        poise::CreateReply::default()
            .content(content)
            .ephemeral(ephemeral)
    };
    ctx.send(reply).await?;
    Ok(())
}
