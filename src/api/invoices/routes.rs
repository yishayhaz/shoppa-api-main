use crate::{db::AxumDBExtansion, prelude::*};
use axum::{extract::Path, response::IntoResponse};
use bson::{doc, oid::ObjectId};
use shoppa_core::ResponseBuilder;

pub async fn update_mail_sent(
    db: AxumDBExtansion,
    Path((token, order_id)): Path<(String, ObjectId)>,
) -> HandlerResult {
    if token.is_empty() || token != "token" {
        return Ok(
            ResponseBuilder::<u16>::error("", None, Some("hello stranger 👋"), Some(404))
                .into_response(),
        );
    }

    let filter = doc! {"order_id": order_id };
    let update = doc! {"$set": {"mail_sent": true}};

    let _ = db.update_many_invoices(filter, update, None, None).await?;

    Ok(ResponseBuilder::success(Some("done"), None, Some(200)).into_response())
}
