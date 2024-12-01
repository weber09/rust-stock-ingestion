use reqwest::Error;

pub async fn fetch_stock_prices(api_key: &str, symbol: &str) -> Result<String, Error> {
    let url = format!(
        "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol={}&interval=1min&apikey={}",
        symbol, api_key
    );

    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    Ok(body)
}