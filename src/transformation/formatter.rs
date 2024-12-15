use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::time::error;

#[derive(Deserialize, Debug)]
pub struct MetaData {
    #[serde(rename = "1. Information")]
    pub information: String,
    #[serde(rename = "2. Symbol")]
    pub symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    pub last_refreshed: String,
    #[serde(rename = "4. Interval")]
    pub interval: String,
    #[serde(rename = "5. Output Size")]
    pub output_size: String,
    #[serde(rename = "6. Time Zone")]
    pub time_zone: String,
}

#[derive(Deserialize, Debug)]
pub struct TimeSeriesEntry {
    #[serde(rename = "1. open")]
    pub open: String,
    #[serde(rename = "2. high")]
    pub high: String,
    #[serde(rename = "3. low")]
    pub low: String,
    #[serde(rename = "4. close")]
    pub close: String,
    #[serde(rename = "5. volume")]
    pub volume: String,
}

#[derive(Deserialize, Debug)]
pub struct StockData {
    #[serde(rename = "Meta Data")]
    pub metadata: MetaData,
    #[serde(rename = "Time Series (1min)")]
    pub time_series: std::collections::HashMap<String, TimeSeriesEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StockPrice {
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub price_change: f64,
    pub symbol: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StockPriceList {
    pub prices: Vec<StockPrice>,
}

pub fn transform_stock_data(raw_data: &str) -> Result<StockPriceList, Box<dyn std::error::Error>> {
    let stock_data: StockData = serde_json::from_str(raw_data)?;

    let mut stock_price_list = StockPriceList {
        prices: Vec::new(),
    };

    for (timestamp, entry) in stock_data.time_series.iter() {
        let naive_time = NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S")?;
        let utc_time = naive_time.and_local_timezone(Utc).unwrap().to_rfc3339();
        let stock_price: StockPrice = StockPrice {
            timestamp: utc_time,
            open: entry.open.parse().unwrap(),
            high: entry.high.parse().unwrap(),
            low: entry.low.parse().unwrap(),
            close: entry.close.parse().unwrap(),
            volume: entry.volume.parse().unwrap(),
            price_change: (entry.close.parse::<f64>().unwrap() - entry.open.parse::<f64>().unwrap()) / entry.open.parse::<f64>().unwrap() * 100.0,
            symbol: stock_data.metadata.symbol.clone(),
        };
        stock_price_list.prices.push(stock_price);
    }

    Ok(stock_price_list)
}
