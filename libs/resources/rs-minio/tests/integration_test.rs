use bytes::Bytes;
use rs_minio::MinioBufferClient;
use std::error::Error;
use uuid::Uuid;


struct TestEnvironment {
    client: MinioBufferClient,
    bucket: String,
    objects: Vec<String>,
}

impl TestEnvironment {
    async fn new(bucket_prefix: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        // Generate a unique bucket name using UUID
        let unique_id = Uuid::new_v4();
        let bucket_name = format!("{}-{}", bucket_prefix, unique_id);

        let client = MinioBufferClient::new(
            "http://127.0.0.1:9000",
            "minio-root-user",
            "minio-root-password",
        )
        .await?;
        client.ensure_bucket_exists(&bucket_name).await?;
        Ok(Self {
            client,
            bucket: bucket_name,
            objects: Vec::new(),
        })
    }

    async fn cleanup(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        for object_name in self.objects.drain(..) {
            self.client
                .delete_object(&self.bucket, &object_name)
                .await?;
        }
        self.client.delete_bucket(&self.bucket).await?;
        Ok(())
    }
}

#[tokio::test]
async fn test_ensure_bucket_exists() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut env = TestEnvironment::new("test-bucket").await?;
    env.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_upload_and_download() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut env = TestEnvironment::new("test-bucket").await?;
    let object_name = "test-object";
    let content = b"Hello, MinIO!";
    env.client
        .upload_from_buffer(&env.bucket, object_name, content)
        .await?;

    env.objects.push(object_name.to_string());

    let downloaded = env
        .client
        .download_to_buffer(&env.bucket, object_name)
        .await?;
    assert_eq!(downloaded, Bytes::from_static(b"Hello, MinIO!"));

    env.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_delete_object_and_bucket() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut env = TestEnvironment::new("test-bucket-for-delete").await?;
    let object_name = "test-object-to-delete";
    let content = b"Content to delete";
    env.client
        .upload_from_buffer(&env.bucket, object_name, content)
        .await?;
    env.client.delete_object(&env.bucket, object_name).await?;
    env.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_list_objects() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut env = TestEnvironment::new("test-bucket-list-objects").await?;

    let content = b"Hello, MinIO!";

    // Upload a couple of objects to list later
    let object_names = vec!["1", "2", "3"];
    for object_name_suffix in &object_names {
        let object_name = format!("test-object-{}", object_name_suffix);
        env.client
            .upload_from_buffer(&env.bucket, &object_name, content)
            .await?;

        env.objects.push(object_name); // Track uploaded objects for cleanup
    }

    let listed_objects = env.client.list_objects(&env.bucket).await?;

    for object_name_suffix in &object_names {
        let object_name = format!("test-object-{}", object_name_suffix);
        assert!(
            listed_objects.contains(&object_name),
            "Object {} was not listed",
            object_name
        );
    }

    env.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_download_non_existent_object() -> Result<(), Box<dyn Error + Send + Sync>> {
    let env = TestEnvironment::new("test-non-existent-object").await?;
    let result = env
        .client
        .download_to_buffer(&env.bucket, "non-existent-object")
        .await;

    assert!(
        matches!(result, Err(_)),
        "Expected an error for non-existent object download"
    );
    Ok(())
}
