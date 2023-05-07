use super::StorageClient;
use crate::helpers::env::ENV_VARS;
use bytes::Bytes;

pub async fn upload_file(
    storage_client: &StorageClient,
    public: bool,
    file: Bytes,
    key: String,
    content_type: String,
) {
    let acl = if public { "public-read" } else { "private" };

    let p_o = storage_client
        .put_object()
        .bucket(ENV_VARS.BUCKET_NAME.clone())
        .set_key(Some(key))
        .set_acl(Some(acl.into()))
        .set_body(Some(file.into()))
        .set_content_type(Some(content_type));

    let res = p_o.send().await;

    tracing::info!("Upload result: {:?}", res);
}

pub async fn upload_product_image(storage_client: &StorageClient) {
    let key = String::from("value");

    upload_file(storage_client, true, Bytes::new(), key.clone(), key).await
}
