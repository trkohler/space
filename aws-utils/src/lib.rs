#![feature(unwrap_infallible)]

use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::put_object::PutObjectOutput;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Error::Unhandled;
use aws_sdk_s3::{Client, Error};
use std::path::Path;
use std::process;

pub async fn upload(path: &str, bucket: &str, key: &str) -> Result<PutObjectOutput, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let file = ByteStream::from_path(Path::new(path)).await.unwrap();

    let resp = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(file)
        .send()
        .await?;
    Ok(resp)
}
