use super::StorageClient;
use crate::helpers::env::ENV_VARS;
use bytes::Bytes;

async fn upload_file(storage_client: &StorageClient, public: bool) {

    let acl = if public {
        "public-read"
    } else {
        "private"
    };

    let body = Bytes::from("adad".as_bytes());

    let p_o = storage_client
        .put_object()
        .bucket(ENV_VARS.BUCKET_NAME.clone())
        .set_acl(Some(acl.into()))
        .set_body(Some(body.into()));

}

pub async fn upload_product_image(storage_client: &StorageClient) {
    upload_file(storage_client, true).await
}
