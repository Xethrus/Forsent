use std::io;
use anyhow::Result;
use anyhow::Context;

use reqwest;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

const API_URL: &str = "https://www.alphavantage.co/query";

enum DataSource {
    Local,
    Api,
}

struct Config {
    source: DataSource,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    #[serde(rename = "Time Series (Daily)")]
    time_series_daily: HashMap<String, DailyData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DailyData {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}

#[tokio::main]
fn make_api_call(&str: api_key) -> Result<()> {
    let response = reqwest::get(&format!("{}?function=TIME_SERIES_DAILY&symbol=MSFT&apikey={}", API_URL, api_key))
        .await?
        .json::<ApiResponse>()
        .await?;
    for (date, data) in response.time_series_daily.iter() {
        println!("Date: {}", date);
        println!("Open: {}", data.open);
        println!("High: {}", data.high);
        println!("Low: {}", data.low);
        println!("Close: {}", data.close);
        println!("Volume: {}", data.volume);
        println!("----------------------------");
    }
    Ok(())
}

fn obtain_intention(config: &mut Config) -> Result<()> {
    loop {
        println!("Select an input data form:");
        println!("1. local");
        println!("2. api");
        println!("3. exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .context("Failed to read from stdin")?;

        match input.trim() {
            "1" => handle_data_source(DataSource::Local, config),
            "2" => handle_data_source(DataSource::Api, config),
            "3" => break,
            _ => println!("Invalid option selected, try again."),
        }
    }
    Ok(())
}

fn handle_data_source(source: DataSource, config: &mut Config) {
    match source {
        DataSource::Local => config.source = DataSource::Local,
        DataSource::Api => config.source = DataSource::Api,
    }
}

fn main() {
    let mut config = Config {
        source: DataSource::Local, // default to Local
    };

    if let Err(e) = obtain_intention(&mut config) {
        eprintln!("An error occurred: {}", e);
    }

    println!("Data source selected: {:?}", config.source);
}

