mod upload;

use crate::helpers::env::ENV_VARS;
use aws_sdk_s3 as s3;

pub type StorageClient = s3::Client;
pub use upload::*;

pub async fn connect() -> StorageClient {
    // for some reason you cant spacify the creadtinal directly
    // https://github.com/awsdocs/aws-rust-developer-guide-v1/blob/main/doc_source/credentials.md#specifying-your-credentials-and-default-region
    std::env::set_var("AWS_ACCESS_KEY_ID", &ENV_VARS.DIGITAL_OCEAN_SPACE_KEY);
    std::env::set_var(
        "AWS_SECRET_ACCESS_KEY",
        &ENV_VARS.DIGITAL_OCEAN_SPACE_SECRET,
    );

    let config = aws_config::from_env()
        .endpoint_url(format!(
            "https://{}.digitaloceanspaces.com",
            &ENV_VARS.DIGITAL_OCEAN_SPACE_REGION
        ))
        .region(aws_sdk_s3::config::Region::new(
            &ENV_VARS.DIGITAL_OCEAN_SPACE_REGION,
        ))
        .load()
        .await;

    let client = StorageClient::new(&config);

    client
}
