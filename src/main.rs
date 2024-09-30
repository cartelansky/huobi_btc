use reqwest;
use serde_json::Value;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.huobi.pro/v1/common/symbols";
    let response = reqwest::get(url).await?.text().await?;
    let json: Value = serde_json::from_str(&response)?;

    let mut markets: Vec<String> = Vec::new();

    if let Some(data) = json["data"].as_array() {
        for item in data {
            if let (Some(base_currency), Some(quote_currency)) = (
                item["base-currency"].as_str(),
                item["quote-currency"].as_str(),
            ) {
                if quote_currency == "btc" {
                    markets.push(format!("HUOBI:{}BTC", base_currency.to_uppercase()));
                }
            }
        }
    }

    markets.sort_by(|a, b| {
        let a_parts: Vec<&str> = a.split(':').collect();
        let b_parts: Vec<&str> = b.split(':').collect();
        let a_coin = a_parts[1].trim_end_matches("BTC");
        let b_coin = b_parts[1].trim_end_matches("BTC");

        fn extract_number(s: &str) -> Option<f64> {
            s.chars()
                .take_while(|c| c.is_digit(10) || *c == '.')
                .collect::<String>()
                .parse()
                .ok()
        }

        match (extract_number(a_coin), extract_number(b_coin)) {
            (Some(a_num), Some(b_num)) => b_num.partial_cmp(&a_num).unwrap_or(Ordering::Equal),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => a_coin.cmp(b_coin),
        }
    });

    let mut file = File::create("huobi_btc_markets.txt")?;
    for market in markets {
        writeln!(file, "{}", market)?;
    }

    println!("Veriler huobi_btc_markets.txt dosyasına yazıldı.");
    Ok(())
}
