mod ingestion;
mod transformation;

use dotenv::dotenv;
use ingestion::api::fetch_stock_prices;
use std::env;
use std::fs::File;
use std::io::Write;
use transformation::formatter::{transform_stock_data, StockPriceList};

#[tokio::main]
async fn main() {
    let api_key = &get_api_key();
    let symbol = "AMZN";

    match fetch_stock_prices(api_key, symbol).await {
        Ok(data) => {
            match transform_stock_data(&data) {
                Ok(stock_data) => {
                    if let Err(err) = save_response_to_file(stock_data, "stock_prices.json").await {
                        eprintln!("Error saving response to file: {}", err);
                    } else {
                        println!("Data saved to stock_prices.json");
                    }
                }
                Err(e) => println!("Error transforming stock data: {}", e),
            }
            /*if let Err(err) = save_response_to_file(&data, "stock_prices.json").await {
                eprintln!("Error saving response to file: {}", err);
            } else {
                println!("Data saved to stock_prices.json");
            }*/
        }
        Err(e) => println!("Error fetching stock data: {}", e),
    }
}

fn get_api_key() -> String {
    dotenv().ok();
    env::var("API_KEY").expect("API_KEY must be set")
}

async fn save_response_to_file(data: StockPriceList, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    let json_data = serde_json::to_string_pretty(&data)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}
