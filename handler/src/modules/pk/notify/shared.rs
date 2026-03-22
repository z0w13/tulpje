use pkrs_fork::{client::PkClient, model::PkId};

use reqwest::StatusCode;
use tulpje_framework::Error;

use crate::{
    context::CommandContext,
    modules::pk::{
        db::{self, ModPkSystem},
        util::SystemRef,
    },
    util::error_response,
};

// TODO: Fetch from DB first, and only fetch from PK if outdated
pub(super) async fn resolve_system_from_reference(
    system_ref: &SystemRef,
    pk_client: &PkClient,
    db: &sqlx::PgPool,
) -> Result<Option<ModPkSystem>, Error> {
    match pk_client.get_system(&PkId(system_ref.clone().into())).await {
        Ok(system) => Ok(Some(ModPkSystem {
            id: system.id.0,
            uuid: system.uuid,
            name: system.name,
        })),
        Err(err)
            if err
                .status()
                .is_some_and(|status| status == StatusCode::NOT_FOUND) =>
        {
            match system_ref {
                SystemRef::Id(_) | SystemRef::Uuid(_) => Ok(db::get_system(db, system_ref).await?),
                SystemRef::DiscordId(_) => Err(
                    "something went wrong, please try deleting using a system ID instead".into(),
                ),
            }
        }
        Err(err) => Err(err.into()),
    }
}

/// try to parse a system ref, and let the end user know if it fails
/// returns None if failed to parse
pub(super) async fn handle_system_ref(
    ctx: &CommandContext,
    system_ref: &str,
) -> Result<Option<SystemRef>, Error> {
    match system_ref.parse() {
        Ok(system_ref) => Ok(Some(system_ref)),
        Err(_) => {
            error_response(
                ctx,
                &format!(
                    "Invalid system reference `{system_ref}`, are you sure you entered it correctly?",
                ),
            )
            .await?;
            Ok(None)
        }
    }
}
