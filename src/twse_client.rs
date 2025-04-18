use dotenvy::dotenv;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::env;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct MiIndex4 {
    pub Date: String,
    pub TradeValue: String,
    pub FormosaIndex: String,
    pub Change: String,
}

pub async fn fetch_mi_index4_json() -> Result<Vec<MiIndex4>, Error> {
    let url = get_api_url();

    // 1. 建立 client
    let client = Client::new();

    // 2. 發 GET 請求，並帶上相同的 header
    let response = client
        .get(url)
        .header("accept", "application/json")
        .header("If-Modified-Since", "Mon, 26 Jul 1997 05:00:00 GMT")
        .header("Cache-Control", "no-cache")
        .header("Pragma", "no-cache")
        .send()
        .await?;

    // println!("🌐 API 狀態碼: {}", response.status());
    // println!("🌐 API Headers: {:#?}", response.headers());

    Ok(response.json::<Vec<MiIndex4>>().await?)
}

pub fn get_api_url() -> String {
    dotenv().ok(); // 載入 .env 檔案
    env::var("URL").expect("URL 環境變數未設定")
}
