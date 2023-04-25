use super::prelude::*;
use crate::db::models;
use crate::prelude::*;

type GetContactForms = Result<Vec<models::ContactUsForm>>;

pub async fn get_contact_us_forms(
  db: &DBExtension, 
  pagination: Option<Pagination>,
  sorting: Option<Sorter>,
  status: Option<models::ContactFormStatus>
) -> Result<Vec<Document>> {

    let pagination = pagination.unwrap_or_default();
    let sorting = sorting.unwrap_or_default();


    let query = match status {
      Some(t) => doc! {
        "status": Into::<Bson>::into(t)
      },
      None => doc! {}
    };

    let pipeline = [
      aggregations::sort(sorting.into()),
      aggregations::skip(pagination.offset),
      aggregations::limit(pagination.amount),
      aggregations::match_query(&query),
      aggregations::add_fields(
        doc! {
          "_id": aggregations::convert_to_string_safe("$_id"),
          "created_at": aggregations::convert_to_string_safe("$created_at"),
          "updated_at": aggregations::convert_to_string_safe("$updated_at"),
        }
      )
    ];

    let cursor = db
        .contact_us_form
        .aggregate(pipeline, None)
        .await
        .map_err(|e| Error::DBError(("contact_us", e)))?;

    let forms = consume_cursor(cursor)
        .await
        .map_err(|e| Error::DBError(("contact_us", e)))?;

    Ok(forms)
}
