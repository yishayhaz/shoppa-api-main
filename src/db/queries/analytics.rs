use super::prelude::*;
use crate::prelude::*;

pub async fn get_views_count(
    db: &DBExtension,
) -> Result<u64> {
  
  let count = db
    .site_visits
    .count_documents(None, None)
    .await
    .map_err(|e| Error::DBError(("site_visits", e)))?;

  Ok(count)

}