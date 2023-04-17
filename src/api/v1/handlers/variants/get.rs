use super::super::prelude::routes::*;
use crate::db::queries;

pub async fn get_variants(db: DBExtension) -> HandlerResponse {
    let variants = queries::get_variants_for_extarnel(&db).await?;

    Ok(ResponseBuilder::paginated_response(&variants).into_response())
}