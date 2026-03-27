use tulpje_framework::Error;
use tulpje_lib::context::TaskContext;

use super::db;

pub(crate) async fn cleanup_systems(ctx: TaskContext) -> Result<(), Error> {
    let deleted = db::cleanup_systems(&ctx.services.db).await?;
    tracing::info!("pk:cleanup-systems cleaned up {deleted} systems");

    Ok(())
}
