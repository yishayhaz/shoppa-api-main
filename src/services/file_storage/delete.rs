use super::StorageClient;
use crate::helpers::env::ENV_VARS;
use aws_sdk_s3::types::{Delete, ObjectIdentifier};

pub async fn delete_files(keys: impl Into<Vec<String>>, storage_client: &StorageClient) {
    let keys: Vec<ObjectIdentifier> = keys
        .into()
        .into_iter()
        .map(|key| ObjectIdentifier::builder().key(key).build())
        .collect();

    let delete = Delete::builder().set_objects(Some(keys)).build();

    let _ = storage_client
        .delete_objects()
        .bucket(ENV_VARS.BUCKET_NAME.clone())
        .delete(delete)
        .send()
        .await;

    tracing::info!("Files deleted");
}
