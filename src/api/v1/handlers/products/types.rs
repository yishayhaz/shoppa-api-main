use crate::prelude::{types::*, *};
use axum::{extract::Multipart, async_trait};


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

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct UploadPayload {
    pub file: Vec<u8>,
}

// #[async_trait]
// impl TryFrom<Multipart> for UploadPayload {
//     type Error = Error;

//     async fn try_from(multipart: Multipart) -> Result<Self> {
//         while let Some(field) = multipart.next_field().await.unwrap() {
//             let name = field.name().unwrap().to_string();
//             let file_name = field.file_name().unwrap().to_string();
//             let content_type = field.content_type().unwrap().to_string();
//             let data = field.bytes().await.unwrap();
    
//             println!(
//                 "Length of `{}` (`{}`: `{}`) is {} bytes",
//                 name,
//                 file_name,
//                 content_type,
//                 data.len()
//             );
//         }
//         Ok(UploadPayload{
//             file: vec![],
//         })
//     }
// }
