use serde::Deserialize;
use std::fs::File;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Bitcoin {
    api_address: String,
    file_name: String,
}

#[derive(Debug)]
enum ApiResult<T> {
    Success(T),
    ApiError(String),
    NetworkError(String),
}

pub trait Pricing {
    fn fetch_price(&self) -> ApiResult<f32>;
    fn save_to_file(&self, price: f32) -> ApiResult<()>;
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> ApiResult<f32> {
        let response = ureq::get(&self.api_address).call();
        match response {
            Ok(btc_price) => {
                if btc_price.status() == 200 {
                    match btc_price.into_json::<BTCPriceAPI>() {
                        Ok(data) => ApiResult::Success(data.bitcoin.usd),
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", btc_price.status()))
                }
            }
            Err(e) => ApiResult::NetworkError(format!("Request failed: {}", e)),
        }
    }

    fn save_to_file(&self, price: f32) -> ApiResult<()> {
        match File::create(&self.file_name) {
            Ok(mut file) => {
                let price_string = format!("Bitcoin price: ${}", price);
                match file.write_all(price_string.as_bytes()) {
                    Ok(_) => ApiResult::Success(()),
                    Err(e) => ApiResult::ApiError(format!("Failed to write to file: {}", e)),
                }
            }
            Err(e) => ApiResult::ApiError(format!("Failed to create file: {}", e)),
        }
    }
}

#[derive(Debug)]
struct Ethereum {
    api_address: String,
    file_name: String,
}
impl Pricing for Ethereum {
    fn fetch_price(&self) -> ApiResult<f32> {
        let response = ureq::get(&self.api_address).call();
        match response {
            Ok(eth_price) => {
                if eth_price.status() == 200 {
                    match eth_price.into_json::<ETHPriceAPI>() {
                        Ok(data) => ApiResult::Success(data.ethereum.usd),
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", eth_price.status()))
                }
            }
            Err(e) => ApiResult::NetworkError(format!("Request failed: {}", e)),
        }
    }

    fn save_to_file(&self, price: f32) -> ApiResult<()> {
        match File::create(&self.file_name) {
            Ok(mut file) => {
                let price_string = format!("Ethereum price: ${}", price);
                match file.write_all(price_string.as_bytes()) {
                    Ok(_) => ApiResult::Success(()),
                    Err(e) => ApiResult::ApiError(format!("Failed to write to file: {}", e)),
                }
            }
            Err(e) => ApiResult::ApiError(format!("Failed to create file: {}", e)),
        }
    }
}

#[derive(Debug)]
struct SP500 {
    api_address: String,
    file_name: String,
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> ApiResult<f32> {
        let response = ureq::get(&self.api_address).call();
        match response {
            Ok(sp500_price) => {
                if sp500_price.status() == 200 {
                    match sp500_price.into_json::<SP500YahooResponse>() {
                        Ok(data) => {
                            let result = &data.chart.result[0];
                            let quote = &result.indicators.quote[0];
                            let close_prices = &quote.close;
                            if let Some(price) = close_prices.last() {
                                if let Some(price) = price {
                                    ApiResult::Success(*price)
                                } else {
                                    ApiResult::ApiError("No valid closing price found".to_string())
                                }
                            } else {
                                ApiResult::ApiError("Failed to find closing price in JSON response".to_string())
                            }
                        }
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", sp500_price.status()))
                }
            }
            Err(e) => ApiResult::NetworkError(format!("Request failed: {}", e)),
        }
    }

    fn save_to_file(&self, price: f32) -> ApiResult<()> {
        match File::create(&self.file_name) {
            Ok(mut file) => {
                let price_string = format!("S&P 500 price: ${}", price);
                match file.write_all(price_string.as_bytes()) {
                    Ok(_) => ApiResult::Success(()),
                    Err(e) => ApiResult::ApiError(format!("Failed to write to file: {}", e)),
                }
            }
            Err(e) => ApiResult::ApiError(format!("Failed to create file: {}", e)),
        }
    }
}

#[derive(Debug, Deserialize)]
struct SP500YahooResponse {
    chart: Chart,
}

#[derive(Debug, Deserialize)]
struct Chart {
    result: Vec<ResultData>,
}

#[derive(Debug, Deserialize)]
struct ResultData {
    indicators: Indicators,
}

#[derive(Debug, Deserialize)]
struct Indicators {
    quote: Vec<Quote>,
}

#[derive(Debug, Deserialize)]
struct Quote {
    close: Vec<Option<f32>>,
}

#[derive(Debug, Deserialize)]
struct Cost {
    usd: f32,
}

#[derive(Debug, Deserialize)]
struct BTCPriceAPI {
    bitcoin: Cost,
}
#[derive(Debug, Deserialize)]
struct ETHPriceAPI {
    ethereum: Cost,
}

fn main() {
    let assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin {
            api_address: "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd".to_string(),
            file_name: "btc_prices.json".to_string(),
        }),
        Box::new(Ethereum {
            api_address: "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd".to_string(),
            file_name: "eth_prices.json".to_string(),
        }),
        Box::new(SP500 {
            api_address: "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d".to_string(),
            file_name: "sp500_prices.json".to_string(),
        }),
    ];

    loop {
        for asset in &assets {
            match asset.fetch_price() {
                ApiResult::Success(price) => {
                    println!("Price: ${}", price);
                    match asset.save_to_file(price) {
                        ApiResult::Success(_) => println!("Price saved successfully."),
                        ApiResult::ApiError(e) => println!("Error saving price: {}", e),
                        ApiResult::NetworkError(e) => println!("Network error while saving price: {}", e),
                    }
                }
                ApiResult::ApiError(e) => println!("API Error fetching price: {}", e),
                ApiResult::NetworkError(e) => println!("Network Error fetching price: {}", e),
            }
        }
        println!("Waiting for 10 seconds...");
        thread::sleep(Duration::from_secs(10));
    }
}
