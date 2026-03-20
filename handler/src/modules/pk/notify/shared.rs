use pkrs_fork::{
    client::PkClient,
    model::{PkId, System},
};

use tulpje_framework::Error;

pub(super) async fn resolve_system_from_reference(system_ref: String) -> Result<System, Error> {
    let client = PkClient::default();
    Ok(client.get_system(&PkId(system_ref)).await?)
}
