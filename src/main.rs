use anyhow::{Context, Result};
use axum::{Router, routing::get};
use clap::Parser;
use tracing::info;

#[derive(Parser)]
#[command(version, about, long_about)]
struct Args {
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    ip: String,

    #[arg(short, long, default_value_t = 10000)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    init_logging();

    let app = Router::new().route("/", get(root));

    let addr = format!("{}:{}", args.ip, args.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context(format!("couldn't bind tcp listener on {addr}"))?;

    info!("started http server: http://{addr}");

    axum::serve(listener, app).await?;

    Ok(())
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}

async fn root() -> &'static str {
    "Hello, there! Welcome to Charst."
}
