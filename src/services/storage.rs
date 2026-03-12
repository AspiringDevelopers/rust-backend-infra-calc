#![allow(dead_code)]

use crate::config::AppConfig;
use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::delete_object::DeleteObjectError;
use aws_sdk_s3::operation::put_object::PutObjectError;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use aws_smithy_types::error::metadata::ProvideErrorMetadata;

pub struct StorageService {
    client: Client,
    bucket: String,
}

impl StorageService {
    pub async fn new(config: &AppConfig) -> Self {
        // Note: In a real implementation, you'd want to properly configure AWS credentials
        let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);

        Self {
            client,
            bucket: config.s3_bucket.clone(),
        }
    }

    pub async fn put_object(&self, key: &str, data: &[u8]) -> Result<(), SdkError<PutObjectError>> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(data.to_vec()))
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_object(&self, key: &str) -> Result<Vec<u8>> {
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        let data = resp.body.collect().await?;
        Ok(data.into_bytes().to_vec())
    }

    pub async fn delete_object(&self, key: &str) -> Result<(), SdkError<DeleteObjectError>> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        Ok(())
    }

    pub async fn object_exists(&self, key: &str) -> anyhow::Result<bool> {
        match self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(SdkError::ServiceError(inner)) => {
                if inner.err().code() == Some("NotFound") {
                    Ok(false)
                } else {
                    Err(anyhow::anyhow!("S3 service error: {:?}", inner))
                }
            }
            Err(e) => Err(anyhow::anyhow!("S3 request error: {:?}", e)),
        }
    }
}
