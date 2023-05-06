use crate::{prelude::{types::*, *}, helpers::extractors::{FromMultipart, FileField}};
use axum::{extract::Multipart, async_trait};
use bytes::Bytes;


#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct CreateProductPayload {
    #[validate(length(min = 8, max = 64))]
    pub name: String,
    // 3 categories must be provided
    #[validate(length(min = 3, max = 3))]
    pub categories: Vec<ObjectId>,

    pub variants: Option<Vec<ObjectId>>,
    pub store: ObjectId,
    pub keywords: Option<Vec<String>>,
    pub brand: Option<String>,
    #[validate(length(min = 8))]
    pub description: String,
}

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
pub struct GetProductsCountQueryParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub store_id: Option<ObjectId>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub category_id: Option<ObjectId>,
}

#[derive(Debug, Clone, Validate)]
pub struct UploadPayload {
    pub file: FileField,
}

#[async_trait]
impl FromMultipart for UploadPayload {
    async fn from_multipart(mut multipart: Multipart) -> Result<Self> {
        let mut file: Option<FileField> = None;

        // TODO improve error handling
        while let Some(field) = multipart.next_field().await.map_err(|e|Error::Static("No field"))? {
            let name = field.name().unwrap_or_default().to_string();

            if name == "file" {
                let content_type = field.content_type();
                tracing::info!("Content type: {:?}", content_type);
                let file_name = field.file_name().unwrap_or_default().to_string();
                tracing::info!("File name: {:?}", file_name);
                let data = field.bytes().await.unwrap();
                tracing::info!("File size: {:?}", data.len());
            }

        }
        Err(Error::Static("No file field"))
    }
}