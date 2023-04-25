use crate::{
    db::queries,
    prelude::{handlers::*, *},
};

pub async fn get_variants(db: DBExtension) -> HandlerResult {
    let variants = queries::get_variants_for_extarnel(&db).await?;

    Ok(ResponseBuilder::paginated_response(&variants).into_response())
}
