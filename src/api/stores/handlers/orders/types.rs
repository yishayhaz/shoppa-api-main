use shoppa_core::{
    constans::{self, MAX_IMAGE_SIZE},
    db::models::OrderPartStatus,
    extractors::{FileFieldstr, FromMultipart},
    parser::empty_string_as_none,
    validators::image_file_field_validator,
};
use crate::prelude::{types::*, *};
use axum::{async_trait, extract::Multipart};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct UpdateOrderStatusPayload {
    pub status: OrderPartStatus,
}