use anyhow::{Context, Result};
use aws_sdk_s3::Client;
use aws_types::region::Region;
use std::path::Path;
use tokio::fs;

pub async fn from_s3(s3_url: &str, dest: &Path) -> Result<()> {
    let (bucket, key) = parse_s3(s3_url)?;

    // All game builds currently live in Mumbai S3
    let bucket_region = std::env::var("GAME_BUCKET_REGION")
        .unwrap_or_else(|_| "ap-south-1".to_string());

    let config = aws_config::defaults(
        aws_config::BehaviorVersion::latest(),
    )
    .region(Region::new(bucket_region))
    .load()
    .await;

    let client = Client::new(&config);

    println!(
        "[S3] Downloading s3://{}/{}",
        bucket,
        key
    );

    let obj = client
        .get_object()
        .bucket(&bucket)
        .key(&key)
        .send()
        .await
        .context("S3 download failed")?;

    let bytes = obj
        .body
        .collect()
        .await?
        .into_bytes();

    fs::write(dest, bytes).await?;

    println!(
        "[S3] Download complete -> {}",
        dest.display()
    );

    Ok(())
}

fn parse_s3(url: &str) -> Result<(String, String)> {
    let u = url
        .strip_prefix("s3://")
        .context("Invalid S3 URL")?;

    let mut parts = u.splitn(2, '/');

    Ok((
        parts
            .next()
            .context("Missing bucket")?
            .to_string(),
        parts
            .next()
            .context("Missing key")?
            .to_string(),
    ))
}