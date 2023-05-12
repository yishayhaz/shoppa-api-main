use super::prelude::*;
use crate::{db::models::SiteVisit, prelude::*};

type InsertSiteVisitResult = Result<SiteVisit>;

pub async fn new_site_visit(db: &DBExtension, ip_address: String) -> InsertSiteVisitResult {
    let mut site_visit = SiteVisit::new(ip_address);

    let res = db
        .site_visits
        .insert_one(&site_visit, None)
        .await
        .map_err(|e| Error::DBError(("site_visits", e)))?;
    
    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(Error::Static("TODO"));
        }
    };

    site_visit.update_id(id);

    Ok(site_visit)
}
