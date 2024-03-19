use bytes::Bytes;
use log::info;
use minio::s3::args::{GetObjectArgs, UploadObjectArgs};
use minio::s3::args::{ListObjectsV2Args, MakeBucketArgs};
use minio::s3::args::{RemoveBucketArgs, RemoveObjectArgs};
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use std::error::Error;
use std::io::Write;
use tempfile::NamedTempFile;

pub struct MinioBufferClient {
    client: Client,
}

impl MinioBufferClient {
    pub async fn new(
        endpoint: &str,
        access_key: &str,
        secret_key: &str,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let base_url: BaseUrl = endpoint.parse()?;
        let static_provider = StaticProvider::new(access_key, secret_key, None);
        let client = Client::new(base_url, Some(Box::new(static_provider)), None, None)?;

        Ok(Self { client })
    }

    pub async fn ensure_bucket_exists(
        &self,
        bucket: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let args = MakeBucketArgs::new(bucket).map_err(Box::new)?;
        match self.client.make_bucket(&args).await {
            Ok(_) => {
                info!("Bucket '{}' created successfully.", bucket);
            }
            Err(minio::s3::error::Error::S3Error(ref s3_error))
                if s3_error.code == "BucketAlreadyOwnedByYou"
                    || s3_error.code == "BucketAlreadyExists" =>
            {
                // The bucket already exists or is owned by you.
                info!("Bucket '{}' already exists or is owned by you.", bucket);
            }
            Err(e) => {
                // An error occurred that wasn't related to the bucket's existence.
                return Err(Box::new(e));
            }
        }

        Ok(())
    }

    pub async fn upload_from_buffer(
        &self,
        bucket: &str,
        object_name: &str,
        buffer: &[u8],
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(buffer)?;

        let temp_path = temp_file
            .path()
            .to_str()
            .ok_or("Failed to get temp file path")?;

        let args = UploadObjectArgs::new(bucket, object_name, temp_path)?;
        self.client.upload_object(&args).await?;
        info!("Uploaded {} to {}/{}", object_name, bucket, object_name);

        Ok(())
    }

    pub async fn download_to_buffer(
        &self,
        bucket: &str,
        object_name: &str,
    ) -> Result<Bytes, Box<dyn Error + Send + Sync>> {
        let args = GetObjectArgs::new(bucket, object_name)?;
        let response = self.client.get_object(&args).await?;
        let bytes = response.bytes().await?;
        Ok(bytes)
    }

    pub async fn delete_object(
        &self,
        bucket: &str,
        object_name: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Properly handle the Result here, unwrapping the actual arguments on success
        let args = RemoveObjectArgs::new(bucket, object_name)?;
        // Pass the unwrapped args reference directly
        self.client.remove_object(&args).await?;
        Ok(())
    }

    pub async fn delete_bucket(&self, bucket: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Properly handle the Result here, unwrapping the actual arguments on success
        let args = RemoveBucketArgs::new(bucket)?;
        // Pass the unwrapped args reference directly
        self.client.remove_bucket(&args).await?;
        Ok(())
    }

    pub async fn list_objects(
        &self,
        bucket: &str,
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        // Attempt to create the arguments for listing objects in the bucket
        let list_obj_args = ListObjectsV2Args::new(bucket)
            .map_err(|e| format!("Failed to create ListObjectsV2Args: {}", e))?;

        // Call the list_objects_v2 method of the MinIO client library
        let response = self.client.list_objects_v2(&list_obj_args).await?;

        // Extract the names of the objects from the response
        let object_names: Vec<String> = response
            .contents
            .iter()
            .map(|entry| entry.name.clone()) // Corrected field name to 'name'
            .collect();

        Ok(object_names)
    }
}
