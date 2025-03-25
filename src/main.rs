use clap::Parser;
use reqwest;
use serde::Deserialize;

/// Currency Converter - Convert between currencies using live exchange rates
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Base currency (e.g., USD)
    #[arg(short, long)]
    from: String,

    /// Target currency (e.g., EUR)
    #[arg(short, long)]
    to: String,

    /// Amount to convert
    #[arg(short, long)]
    amount: f64,
}

#[derive(Deserialize)]
struct ApiResponse {
    result: f64,
    info: RateInfo,
}

#[derive(Deserialize)]
struct RateInfo {
    rate: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let url = format!(
        "https://api.exchangerate.host/convert?from={}&to={}&amount={}",
        args.from, args.to, args.amount
    );

    let response: ApiResponse = reqwest::get(&url).await?.json().await?;

    println!(
        "{} {} = {:.2} {} (Rate: {:.4})",
        args.amount, args.from.to_uppercase(), response.result, args.to.to_uppercase(), response.info.rate
    );

    Ok(())
}
