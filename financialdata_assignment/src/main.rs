use std::{thread, time::Duration};
use chrono::Utc;
use serde::Deserialize;
use ureq;

// Trait Definition
trait Pricing {
    // Fetches the last price for the asset
    fn fetch_price(&mut self) -> Result<(), String>;
    
    // Prints price to the terminal
    fn print_price(&self);
}

// Struct Definitions for Assets
struct Bitcoin {
    price: f64,
}

struct Ethereum {
    price: f64,
}

struct SP500 {
    price: f64,
}

// API Response Structs for Deserialization
#[derive(Deserialize, Debug)]
struct CombinedPriceResponse {
    bitcoin: CurrencyData,
    ethereum: CurrencyData,
}

#[derive(Deserialize, Debug)]
struct CurrencyData {
    usd: f64, // USD price
}

#[derive(Deserialize, Debug)]
struct YahooChart {
    chart: YahooResult,
}

#[derive(Deserialize, Debug)]
struct YahooResult {
    result: Option<Vec<YahooMetaWrapper>>,
}

#[derive(Deserialize, Debug)]
struct YahooMetaWrapper {
    meta: YahooMeta,
}

#[derive(Deserialize, Debug)]
struct YahooMeta {
    regularMarketPrice: f64, // S&P 500 market price
}

// Shared JSON Fetching Helper Function
fn get_json<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, String> {
    ureq::get(url)
        .call()
        .map_err(|e| format!("HTTP error: {}", e))?
        .into_json()
        .map_err(|e| format!("JSON parse error: {}", e))
}

// Shared Crypto Fetcher Function
fn fetch_crypto_prices() -> Result<(f64, f64), String> {
    // Fetch prices for Bitcoin and Ethereum
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd";
    let response: CombinedPriceResponse = get_json(url)?;
    Ok((response.bitcoin.usd, response.ethereum.usd))
}

// Implement Pricing Trait for Bitcoin
impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<(), String> {
        // Bitcoin fetch handled outside (already fetched in main loop)
        Ok(())
    }

    fn print_price(&self) {
        // Print Bitcoin prices
        println!("Bitcoin: ${:.2}", self.price);
    }
}

//  Implement Pricing Trait for Ethereum 
impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<(), String> {
        // Ethereum fetch handled outside (already fetched in main loop)
        Ok(())
    }

    fn print_price(&self) {
        // Prints Ethereum price
        println!("Ethereum: ${:.2}", self.price);
    }
}

//  Implement Pricing Trait for SP500 
impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<(), String> {
        // Fetch S&P 500 price from Yahoo Finance API
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m";
        let response: YahooChart = get_json(url)?;
        self.price = response.chart
            .result
            .ok_or("Missing Array Result!")?
            .get(0)
            .ok_or("No Data!")?
            .meta
            .regularMarketPrice;
        Ok(())
    }

    fn print_price(&self) {
        // Print the S&P 500 price to the terminal
        println!("S&P 500: ${:.2}", self.price);
    }
}

//  Main Loop to Fetch and Print Prices 
fn main() {
    let mut sp500 = SP500 { price: 0.0 };

    loop {
        // Print timestamp indicating start of data fetch
        println!(" Fetching prices @ {} ", Utc::now());

        //  Fetch Bitcoin & Ethereum in ONE request 
        match fetch_crypto_prices() {
            Ok((btc_price, eth_price)) => {
                let btc = Bitcoin { price: btc_price };
                let eth = Ethereum { price: eth_price };

                // Print the fetched Bitcoin and Ethereum prices
                btc.print_price();
                eth.print_price();
            }
            Err(e) => eprintln!("Crypto fetch error: {}", e),
        }

        //  Fetch SP500 Price Separately 
        match sp500.fetch_price() {
            Ok(_) => {
                // Print the fetched S&P 500 price
                sp500.print_price();
            }
            Err(e) => eprintln!("SP500 fetch error: {}", e),
        }

        // Wait for 10 seconds before fetching again
        thread::sleep(Duration::from_secs(10));
    }
}