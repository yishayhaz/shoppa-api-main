use crate::{
    helpers::extractors::{FileFieldstr, FromMultipart},
    prelude::{types::*, *},
};
use axum::{async_trait, extract::Multipart};
// use bytes::Bytes;

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
pub struct UploadProductImagesPayload {
    pub files: Vec<FileFieldstr>,
}

#[async_trait]
impl FromMultipart for UploadProductImagesPayload {
    async fn from_multipart(mut multipart: Multipart) -> Result<Self> {
        let mut files: Vec<FileFieldstr> = vec![];

        // TODO improve
        while let Some(field) = multipart
            .next_field()
            .await
            .map_err(|_| Error::Static("No field"))?
        {
            let name = field.name().unwrap_or_default().to_string();

            if name == "files" {
                let file_name = field.file_name().unwrap_or_default().to_string();
                let content_type = field.content_type().unwrap().to_string();
                let data = field.bytes().await.unwrap();

                files.push(FileFieldstr::new(file_name, content_type, data));
            }
        }
        Ok(Self { files })
    }
}
