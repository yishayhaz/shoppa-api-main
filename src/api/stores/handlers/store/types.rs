use crate::db::DeliveryStrategiesUpdatePayload;
use crate::prelude::{types::*, *};
use axum::{async_trait, extract::Multipart};
use shoppa_core::{
    constans,
    db::models::StoreLocation,
    extractors::{FileFieldstr, FromMultipart},
    parser::FieldPatch,
    validators::{image_file_field_validator, number_string_validator, phone_number_validator},
};
#[derive(Validate)]
pub struct UpdateStoreAssetsPayload {
    #[validate(
        length(max = "constans::MAX_IMAGE_SIZE"),
        custom = "image_file_field_validator"
    )]
    pub logo: Option<FileFieldstr>,
    #[validate(
        length(max = "constans::MAX_IMAGE_SIZE"),
        custom = "image_file_field_validator"
    )]
    pub banner: Option<FileFieldstr>,
}

pub type StoreLocationPayload = StoreLocation;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UpdateStorePayload {
    #[serde(default)]
    #[validate(length(
        min = "constans::STORE_SLOGAN_MIN_LENGTH",
        max = "constans::STORE_SLOGAN_MAX_LENGTH"
    ))]
    pub slogan: FieldPatch<String>,
    #[validate(length(
        min = "constans::STORE_DESCRIPTION_MIN_LENGTH",
        max = "constans::STORE_DESCRIPTION_MAX_LENGTH"
    ))]
    pub description: Option<String>,
    #[validate(email)]
    pub contact_email: Option<String>,
    #[validate(custom = "phone_number_validator")]
    pub contact_phone: Option<String>,
    pub min_order: Option<u64>,
    pub delivery_strategies: Option<DeliveryStrategiesUpdatePayload>,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UpdateStoreLocationPayload {
    #[validate(length(
        min = "constans::CITY_NAME_MIN_LENGTH",
        max = "constans::CITY_NAME_MAX_LENGTH"
    ))]
    pub city: Option<String>,
    #[validate(length(
        min = "constans::STREET_NAME_MIN_LENGTH",
        max = "constans::STREET_NAME_MAX_LENGTH"
    ))]
    pub street: Option<String>,
    #[validate(length(
        min = "constans::STREET_NUMBER_MIN_LENGTH",
        max = "constans::STREET_NUMBER_MAX_LENGTH"
    ))]
    pub street_number: Option<String>,
    #[serde(default)]
    #[validate(length(max = "constans::LOCATION_FREE_TEXT_MAX_LENGTH"))]
    pub free_text: FieldPatch<String>,
    #[validate(custom = "number_string_validator")]
    pub phone: Option<String>,
}

#[async_trait]
impl FromMultipart for UpdateStoreAssetsPayload {
    async fn from_multipart(mut multipart: Multipart) -> Result<Self> {
        let mut logo: Option<FileFieldstr> = None;
        let mut banner: Option<FileFieldstr> = None;

        let mut data_provided: bool = false;

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

            if !data_provided {
                return Err(Error::Static("No data provided"));
            }
        }
        Ok(Self { logo, banner })
    }
}
