use num_format::{Locale, ToFormattedString};
use reqwest::Error;
use serde::{Deserialize, Serialize};

// Written with chatGPT.

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_url = "https://api.kraken.com/0/public/Trades?pair=xbtusd&count=1";

    let response = reqwest::get(api_url).await?;

    if response.status().is_success() {
        let parsed_response: ApiResponse = response.json().await?;
        let price = &parsed_response.result.XXBTZUSD[0][0].to_dollars();
        println!("\n\n");
        println!("Last BTC trade on Kraken: {}", price);
        println!("\n\n");
    } else {
        eprintln!("Failed to fetch data. Status: {}", response.status());
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    error: Vec<String>,
    result: ResultData,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ResultData {
    XXBTZUSD: Vec<Vec<XbtUsdData>>,
    last: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum XbtUsdData {
    String(String),
    Number(f64),
    Integer(u64),
}

impl XbtUsdData {
    fn to_dollars(&self) -> String {
        use XbtUsdData::*;

        match *self {
            String(ref s) => {
                let amount = s.parse::<f64>().unwrap();
                let integer_part = (amount as u64).to_formatted_string(&Locale::en);
                let fractional_part = format!("{:.2}", amount % 1.0);
                format!("${}{}", integer_part, fractional_part.trim_start_matches('0'))
            }
            Number(_) | Integer(_) => panic!("wtf"),
        }
    }
}
