use crate::prelude::{types::*, *};
use crate::{
    db::models::{Store, StoreBusinessType},
    helpers::{
        extractors::{FileFieldstr, FromMultipart},
        validators::{image_file_field_validator, number_string_validator, phone_number_validator},
        MAX_IMAGE_SIZE,
    },
};
use axum::{async_trait, extract::Multipart};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchStoresQueryParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub free_text: Option<String>,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct CreateStorePayload {
    #[validate(length(min = 3, max = 60))]
    pub name: String, // store name
    #[validate(length(min = 8, max = 40))]
    pub slogan: Option<String>,
    #[validate(length(min = 20, max = 160))]
    pub description: String,
    #[validate(email)]
    pub contact_email: String,
    #[validate(custom = "phone_number_validator")]
    pub contact_phone: String,
    #[validate(custom = "number_string_validator")]
    pub legal_id: String,
    pub legal_name: String,
    pub business_type: StoreBusinessType,
}


#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct StoreLocation {

}

#[derive(Validate)]
pub struct UpdateStorePayload {
    #[validate(length(max = "MAX_IMAGE_SIZE"), custom = "image_file_field_validator")]
    pub logo: Option<FileFieldstr>,
    #[validate(length(max = "MAX_IMAGE_SIZE"), custom = "image_file_field_validator")]
    pub banner: Option<FileFieldstr>,
}

#[async_trait]
impl FromMultipart for UpdateStorePayload {
    async fn from_multipart(mut multipart: Multipart) -> Result<Self> {
        let mut logo: Option<FileFieldstr> = None;
        let mut banner: Option<FileFieldstr> = None;

        let mut data_provided: bool = false;

        // TODO improve
        while let Some(field) = multipart
            .next_field()
            .await
            .map_err(Error::MultiPartFormError)?
        {
            if logo.is_some() && banner.is_some() {
                break;
            }

            let name = field.name().unwrap_or_default().to_string();

            if name == "logo" || name == "banner" {
                let file_name = field.file_name().unwrap_or_default().to_string();

                if file_name == "" {
                    return Err(Error::Static("No file name provided"));
                }

                let content_type = field.content_type().unwrap_or_default().to_string();

                let data = field.bytes().await.map_err(Error::MultiPartFormError)?;

                let file_ext = file_name.split(".").last().unwrap_or_default().to_string();

                if file_ext == "" {
                    return Err(Error::Static("No file extension provided"));
                }

                if name == "logo" {
                    logo = Some(FileFieldstr {
                        file_name,
                        content_type,
                        size: data.len(),
                        file: data,
                        file_extension: file_ext,
                    });
                } else {
                    banner = Some(FileFieldstr {
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

impl Into<Store> for CreateStorePayload {
    fn into(self) -> Store {
        Store::new(
            self.name,
            self.description,
            self.contact_email,
            self.contact_phone,
            self.slogan,
            self.legal_id,
            self.business_type,
            self.legal_name,
        )
    }
}
