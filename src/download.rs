use anyhow::{Result, Context};
use aws_sdk_s3::Client;
use std::path::Path;
use tokio::fs;

pub async fn from_s3(s3_url: &str, dest: &Path) -> Result<()> {
    let (bucket, key) = parse_s3(s3_url)?;

    let config = aws_config::load_defaults(
        aws_config::BehaviorVersion::latest()
    ).await;

    let client = Client::new(&config);

    let obj = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .context("S3 download failed")?;

    let bytes = obj.body.collect().await?.into_bytes();
    fs::write(dest, bytes).await?;

    Ok(())
}

fn parse_s3(url: &str) -> Result<(String, String)> {
    let u = url
        .strip_prefix("s3://")
        .context("Invalid S3 URL")?;

    let mut parts = u.splitn(2, '/');
    Ok((
        parts.next().unwrap().to_string(),
        parts.next().unwrap().to_string(),
    ))
}
