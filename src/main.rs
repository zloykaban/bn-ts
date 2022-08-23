use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
//use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ExchangeInfo {
    timezone: String,
    server_time: i64,
    //rate_limits: Vec<RateLimit>,
    rate_limits: Vec<RateLimit>,
    //rate_limits: HashMap<String, RateLimit>,
    exchange_filters: Vec<String>,
    symbols: Vec<Symbol>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RateLimit {
    //enum RateLimit {
    rate_limit_type: String,
    interval: String,
    interval_num: i32,
    limit: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Symbol {
    symbol: String,
    status: String,
    base_asset: String,
    base_asset_precision: i32,
    quote_asset: String,
    quote_precision: i32,
    quote_asset_precision: i32,
    base_commission_precision: i32,
    quote_commission_precision: i32,
    order_types: Vec<String>,
    iceberg_allowed: bool,
    oco_allowed: bool,
    quote_order_qty_market_allowed: bool,
    allow_trailing_stop: bool,
    cancel_replace_allowed: bool,
    is_spot_trading_allowed: bool,
    is_margin_trading_allowed: bool,
    filters: Vec<serde_json::Value>,
    permissions: Vec<String>,
}

impl ExchangeInfo {
    async fn get_exchange_info() -> Result<Self, ExitFailure> {
        let url = "https://api.binance.com/api/v3/exchangeInfo?symbol=BNBBTC".to_string();

        let url = Url::parse(url.as_str())?;

        let res = reqwest::get(url).await?.json::<ExchangeInfo>().await?;

        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let res = ExchangeInfo::get_exchange_info().await?;
    println!("{:?}", res.rate_limits);

    Ok(())
}
