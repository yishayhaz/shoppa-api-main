use crate::{db::models, helpers::types::DBExtension, prelude::*};
use bson::oid::ObjectId;

pub async fn delete_category_by_ids(
  db: &DBExtension,
  ids: &Vec<ObjectId>,
) -> Result<Option<models::Categories>> {
  // Requirements:
  // 1. only when category has no subcategories (no children)
  // 2. and no products are using it
  todo!()
}
