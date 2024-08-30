use crate::db::user_service;
use crate::replies::default_replies;
use crate::types::ctx;

pub async fn leave_cmd(ctx: &ctx::Context<'_>) -> Result<(), ctx::Error> {
    let is_existing: bool = user_service::user_exists(ctx).await;

    if !is_existing {
        default_replies::user_does_not_exist(ctx, true).await?;
        return Ok(());
    }

    default_replies::user_deleted_successfully(ctx, true).await?;
    let _deleting = user_service::delete_user(&ctx.data().mongo_client, ctx);
    Ok(())
}
