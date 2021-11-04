use hyper::{body::HttpBody, Client};
use hyper_tls::HttpsConnector;
use serde_json::Value;

const SOLSCAN_TOKEN_API: &str = "https://api.solscan.io/tokens/?offset=0&limit=5000";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let uri = SOLSCAN_TOKEN_API.parse::<hyper::Uri>().unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut res = client.get(uri).await?;

    let mut body: Vec<u8> = Vec::new();

    while let Some(next) = res.data().await {
        body.extend(next?.to_vec().iter().copied());
    }

    let result: Value = serde_json::from_slice(&body)?;

    let mut sum = 0.0;
    for it in result["data"]["tokens"].as_array().unwrap().into_iter() {
        sum += it["marketCapFD"].as_f64().unwrap_or(0.0);
    }

    println!("Total marketcap: {}", sum);
    println!("Total tokens: {}", result["data"]["total"]);

    Ok(())
}
