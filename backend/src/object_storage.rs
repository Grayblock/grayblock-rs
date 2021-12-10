use anyhow::Result;
use log::info;
use s3::{creds::Credentials, Bucket, Region};

/// Gets data and content type from a path in an object store
pub async fn get(path: &str) -> Result<(Vec<u8>, Option<String>)> {
    info!("GET: {}", path);

    let bucket = Bucket::new(
        "grayblock",
        Region::DoNyc3,
        Credentials::from_env_specific(
            Some("OBJECT_STORE_ACCESS_KEY"),
            Some("OBJECT_STORE_SECRET_KEY"),
            None,
            None,
        )?,
    )?;

    let (data, _) = bucket.get_object(&path).await?;
    let (meta, _) = bucket.head_object(&path).await?;

    Ok((data, meta.content_type))
}
