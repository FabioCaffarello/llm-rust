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

/// A client for interacting with MinIO using in-memory buffers.
pub struct MinioBufferClient {
    client: Client,
}

impl MinioBufferClient {
    /// Creates a new `MinioBufferClient` instance.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The URL to the MinIO service.
    /// * `access_key` - Your MinIO access key.
    /// * `secret_key` - Your MinIO secret key.
    ///
    /// # Returns
    ///
    /// A result containing the new `MinioBufferClient` instance or an error if the client
    /// could not be created.
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

    /// Ensures that a bucket exists in the MinIO service.
    ///
    /// If the bucket does not exist, it is created. If it already exists, the function does nothing.
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the bucket to check or create.
    ///
    /// # Returns
    ///
    /// A result indicating success or an error if the bucket could not be checked or created.
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

    /// Uploads a buffer as an object to a specified bucket.
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the target bucket.
    /// * `object_name` - The name of the object to create.
    /// * `buffer` - The data to upload as the object.
    ///
    /// # Returns
    ///
    /// A result indicating success or an error if the upload failed.
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

    /// Downloads an object from a specified bucket into a buffer.
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the bucket.
    /// * `object_name` - The name of the object to download.
    ///
    /// # Returns
    ///
    /// A result containing the downloaded data as `Bytes` or an error if the download failed.
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

    /// Deletes an object from a specified bucket.
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the bucket.
    /// * `object_name` - The name of the object to delete.
    ///
    /// # Returns
    ///
    /// A result indicating success or an error if the deletion failed.
    pub async fn delete_object(
        &self,
        bucket: &str,
        object_name: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let args = RemoveObjectArgs::new(bucket, object_name)?;
        self.client.remove_object(&args).await?;
        Ok(())
    }

    /// Deletes a specified bucket.
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the bucket to delete.
    ///
    /// # Returns
    ///
    /// A result indicating success or an error if the bucket could not be deleted.
    pub async fn delete_bucket(&self, bucket: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let args = RemoveBucketArgs::new(bucket)?;
        self.client.remove_bucket(&args).await?;
        Ok(())
    }

    /// Lists all objects in a specified bucket.
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the bucket.
    ///
    /// # Returns
    ///
    /// A result containing a vector of object names or an error if the listing failed.
    pub async fn list_objects(
        &self,
        bucket: &str,
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let list_obj_args = ListObjectsV2Args::new(bucket)
            .map_err(|e| format!("Failed to create ListObjectsV2Args: {}", e))?;
        let response = self.client.list_objects_v2(&list_obj_args).await?;
        let object_names: Vec<String> = response
            .contents
            .iter()
            .map(|entry| entry.name.clone())
            .collect();

        Ok(object_names)
    }
}
