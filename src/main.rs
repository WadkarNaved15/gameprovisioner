use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    game_id: String,

    #[arg(long)]
    build_id: String,

    #[arg(long)]
    s3_url: String,

    #[arg(long)]
    format: String, // "7z" | "zip" | "exe"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    game_provisioner::ensure_game_ready(
        &args.game_id,
        &args.build_id,
        &args.s3_url,
        &args.format,
    ).await?;

    println!("Game provisioned successfully");
    Ok(())
}
