use crate::prelude::{types::*, *};
use axum::{async_trait, extract::Multipart};
use shoppa_core::{
    constans::MAX_IMAGE_SIZE,
    extractors::{FileFieldstr, FromMultipart},
    parser::{deserialize_optional_query_array, empty_string_as_none},
    validators::image_file_field_validator,
};
use validator::Validate;

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct GetProductQueryParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub free_text: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub store_id: Option<ObjectId>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub category_id: Option<ObjectId>,
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct GetProductsAutoCompleteQueryParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub free_text: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub store_id: Option<ObjectId>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub category_id: Option<ObjectId>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub amount: Option<u8>,
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct GetProductsCountQueryParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub store_id: Option<ObjectId>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub category_id: Option<ObjectId>,
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct GetProductsInfiniteQueryParams {
    #[serde(deserialize_with = "deserialize_optional_query_array")]
    pub product_ids: Option<Vec<ObjectId>>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub category_id: Option<ObjectId>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub store_id: Option<ObjectId>,
}

#[derive(Debug, Clone, Validate)]
pub struct UploadProductImagePayload {
    #[validate(length(max = "MAX_IMAGE_SIZE"), custom = "image_file_field_validator")]
    pub file: FileFieldstr,
}

#[async_trait]
impl FromMultipart for UploadProductImagePayload {
    async fn from_multipart(mut multipart: Multipart) -> Result<Self> {
        let mut file: Option<FileFieldstr> = None;

        while let Some(field) = multipart.next_field().await? {
            let name = field.name().unwrap_or_default().to_string();

            if name == "files" {
                let file_name = field.file_name().unwrap_or_default().to_string();
                let content_type = field.content_type().unwrap().to_string();
                let data = field.bytes().await.unwrap();

                file = Some(FileFieldstr::new(file_name, content_type, data));
            }
        }

        if let Some(file) = file {
            Ok(Self { file })
        } else {
            Err(Error::NoNewDataProvided)
        }
    }
}
