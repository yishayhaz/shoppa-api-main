use super::prelude::*;
use crate::db::models;

type GetContactForms = Result<Vec<models::ContactUsForm>, Response>;

pub async fn get_contact_us_forms(db: &DBExtension) -> GetContactForms {
  let cursor = match db.contact_us_form.find(None, None).await {
    Ok(cursor) => cursor,
    Err(_) => {
        return Err(ResponseBuilder::<u16>::error(
            // TODO add error code here
            "",
            None,
            Some("Internal Server Error while fetching store"),
            Some(500),
        )
        .into_response())
    }
  };

  let forms = match consume_cursor(cursor).await {
    Ok(forms) => forms,
    Err(_) => {
        return Err(ResponseBuilder::<u16>::error(
            // TODO add error code here
            "",
            None,
            Some("Internal Server Error while fetching forms"),
            Some(500),
        )
        .into_response())
    }
};



  Ok(forms)
}
