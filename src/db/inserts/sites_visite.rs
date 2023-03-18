use super::InsertDocumentErrors;
use crate::{
    db::models::{DBModel, SiteVisit},
    helpers::types::DBExtension,
};
use mongodb::error::ErrorKind;

type InsertSiteVisitResult = Result<SiteVisit, InsertDocumentErrors>;

pub async fn new_site_visit(db: &DBExtension, ip_address: String) -> InsertSiteVisitResult {
    let mut site_visit = SiteVisit::new(ip_address);

    let res = match db.site_visits.insert_one(&site_visit, None).await {
        Ok(v) => v,
        Err(err) => match *err.kind {
            ErrorKind::Write(e) => {
                todo!("find a way to know if its a dup document");
                return Err(InsertDocumentErrors::UnknownError);
            }
            _ => {
                return Err(InsertDocumentErrors::UnknownError);
            }
        },
    };

    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(InsertDocumentErrors::UnknownError);
        }
    };

    site_visit.update_id(id);

    Ok(site_visit)
}
