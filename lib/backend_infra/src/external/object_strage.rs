use aws_sdk_s3::primitives::ByteStream;
use axum::async_trait;

use domain::external::object_strage::ObjectStrageClient;

#[derive(Clone)]
pub struct ObjectStorageClientImpl {
    s3_client: aws_sdk_s3::Client,
    bucket_name: String,
}

impl ObjectStorageClientImpl {
    pub fn new(s3_client: aws_sdk_s3::Client, bucket_name: String) -> Self {
        Self {
            s3_client,
            bucket_name,
        }
    }
}

#[async_trait]
impl ObjectStrageClient for ObjectStorageClientImpl {
    async fn upload(&self, file_name: &str, data: &str) -> anyhow::Result<()> {
        self.s3_client
            .put_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .body(ByteStream::from(data.as_bytes().to_vec()))
            .send()
            .await?;

        Ok(())
    }

    async fn download(&self, file_name: &str) -> anyhow::Result<String> {
        let res = self
            .s3_client
            .get_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .send()
            .await?;

        let body = res.body.collect().await?;
        let body = String::from_utf8(body.to_vec())?;

        Ok(body)
    }

    async fn delete(&self, file_name: &str) -> anyhow::Result<()> {
        self.s3_client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .send()
            .await?;

        Ok(())
    }
}
