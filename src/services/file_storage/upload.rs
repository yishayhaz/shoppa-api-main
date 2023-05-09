use super::StorageClient;
use super::keys;
use crate::helpers::env::ENV_VARS;
use bytes::Bytes;
use bson::oid::ObjectId;

pub async fn upload_file(
    storage_client: &StorageClient,
    public: bool,
    file: Bytes,
    key: &String,
    content_type: &String,
) {
    let acl = if public { "public-read" } else { "private" };

    let p_o = storage_client
        .put_object()
        .bucket(ENV_VARS.BUCKET_NAME.clone())
        .key(key)
        .acl(acl.into())
        .body(file.into())
        .content_type(content_type);

    let res = p_o.send().await;

    tracing::info!("Upload result: {:?}", res);
}

pub async fn upload_product_image(
    storage_client: &StorageClient,
    file: Bytes,
    // TODO create content type enum and a function to get the file extension
    content_type: &String,
    product_id: &ObjectId,
) {

    let key = keys::generate_product_image_key(product_id, content_type);

    upload_file(storage_client, true, file, &key, content_type).await
}

pub async fn upload_store_logo(
    storage_client: &StorageClient,
    file: Bytes,
    content_type: &String,
    store_id: &ObjectId,
    file_extension: &String,
) -> String {

    let key = keys::generate_store_logo_key(store_id, file_extension);

    upload_file(storage_client, true, file, &key, content_type).await;

    key
}

pub async fn upload_store_banner(
    storage_client: &StorageClient,
    file: Bytes,
    content_type: &String,
    store_id: &ObjectId,
    file_extension: &String,
) -> String {

    let key = keys::generate_store_banner_key(store_id, file_extension);

    upload_file(storage_client, true, file, &key, content_type).await;

    key
}
