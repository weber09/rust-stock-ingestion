mod ingestion;

use ingestion::api::fetch_stock_prices;
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() {
   let api_key = &get_api_key();
   let symbol = "AMZN";

   match fetch_stock_prices(api_key, symbol).await {
    Ok(data) => {
        if let Err(err) = save_response_to_file(&data, "stock_prices.json").await {
            eprintln!("Error saving response to file: {}", err);
        } else {
            println!("Data saved to stock_prices.json");
        }
    },
    Err(e) => println!("Error fetching stock data: {}", e),
   }
}

fn get_api_key() -> String {
    dotenv().ok();
    env::var("API_KEY").expect("API_KEY must be set")
}

async fn save_response_to_file(data: &str, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
