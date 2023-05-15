use super::prelude::*;
use crate::{db::models::SiteVisit, prelude::*};
use validator::Validate;

pub async fn new_site_visit<T>(db: &DBExtension, site_visit: T) -> Result<SiteVisit>
where
    T: Into<SiteVisit>,
{
    let mut site_visit: SiteVisit = site_visit.into();

    site_visit.validate()?;

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

pub async fn try_new_site_visit<T>(db: &DBExtension, site_visit: T) -> Result<SiteVisit>
where
    T: TryInto<SiteVisit>,
    T::Error: Into<Error>,
{
    let site_visit = site_visit.try_into().map_err(|e| e.into())?;

    new_site_visit(db, site_visit).await
}

pub async fn new_site_visit_from_ip(db: &DBExtension, ip: String) -> Result<SiteVisit> {
    let site_visit = SiteVisit::new(ip.into());

    new_site_visit(db, site_visit).await
}
