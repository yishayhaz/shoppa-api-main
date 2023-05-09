use crate::helpers::extractors::{FileField, FromMultipart};
use crate::prelude::{types::*, *};
use axum::{async_trait, extract::Multipart};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct CreateStorePayload {
    #[validate(email)]
    pub email: String,
    pub name: String,
    pub description: String,
    pub location: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchStoresQueryParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub free_text: Option<String>,
}

pub struct UpdateStorePayload {
    pub logo: Option<FileField>,
    pub banner: Option<FileField>,
}

#[async_trait]
impl FromMultipart for UpdateStorePayload {
    async fn from_multipart(mut multipart: Multipart) -> Result<Self> {
        let mut logo: Option<FileField> = None;
        let mut banner: Option<FileField> = None;

        let mut data_provided: bool = false;

        // TODO improve
        while let Some(field) = multipart
            .next_field()
            .await
            .map_err(|_| Error::Static("No field"))?
        {
            let name = field.name().unwrap_or_default().to_string();

            if name == "logo" || name == "banner" {
                let file_name = field.file_name().unwrap_or_default().to_string();

                let content_type = field.content_type().unwrap_or_default().to_string();

                let data = field.bytes().await.map_err(|_| Error::Static("No field"))?;

                let file_ext = file_name.split(".").last().unwrap_or_default().to_string();

                if name == "logo" {
                    logo = Some(FileField {
                        file_name,
                        content_type,
                        size: data.len(),
                        file: data,
                        file_extension: file_ext,
                    });
                } else {
                    banner = Some(FileField {
                        file_name,
                        content_type,
                        size: data.len(),
                        file: data,
                        file_extension: file_ext,
                    });
                }

                data_provided = true;
            }
        }

        if !data_provided {
            return Err(Error::Static("No data provided"));
        }

        Ok(Self { logo, banner })
    }
}
