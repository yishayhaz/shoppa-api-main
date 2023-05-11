use super::keys;
use super::StorageClient;
use crate::helpers::env::ENV_VARS;
use bson::oid::ObjectId;
use bytes::Bytes;

pub struct Uploader<'a> {
    public: bool,
    pub key: String,
    content_type: &'a String,
    file: Bytes,
}

impl Uploader<'_> {
    pub fn key(&self) -> &String {
        &self.key
    }

    pub async fn fire(self, storage_client: &StorageClient) {
        let acl = if self.public {
            "public-read"
        } else {
            "private"
        };

        let p_o = storage_client
            .put_object()
            .bucket(ENV_VARS.BUCKET_NAME.clone())
            .key(self.key)
            .acl(acl.into())
            .body(self.file.into())
            .content_type(self.content_type);

        let res = p_o.send().await;

        tracing::info!("Upload result: {:?}", res);
    }
}

pub fn upload_product_image<'a>(
    file: Bytes,
    // TODO create content type enum and a function to get the file extension
    content_type: &'a String,
    product_id: &ObjectId,
    file_extension: &String,
) -> Uploader<'a> {
    let key = keys::generate_product_image_key(product_id, file_extension);

    Uploader {
        public: true,
        key,
        content_type,
        file,
    }
}

pub fn upload_store_logo<'a>(
    file: Bytes,
    content_type: &'a String,
    store_id: &ObjectId,
    file_extension: &String,
) -> Uploader<'a> {
    let key = keys::generate_store_logo_key(store_id, file_extension);

    Uploader {
        public: true,
        key,
        content_type,
        file,
    }
}

pub fn upload_store_banner<'a>(
    file: Bytes,
    content_type: &'a String,
    store_id: &ObjectId,
    file_extension: &String,
) -> Uploader<'a> {
    let key = keys::generate_store_banner_key(store_id, file_extension);

    Uploader {
        public: true,
        key,
        content_type,
        file,
    }
}
