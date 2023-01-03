extern crate chapa_rust;

use dotenv::dotenv;
use reqwest::header::{AUTHORIZATION, HeaderMap};
use std::env;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Bank {
    id:String,
    swift:String,
    name:String,
    acct_length:u32,
    country_id:u32,
    created_at: String,
    updated_at: String,
    is_mobilemoney: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct BankRequestResponse {
    message: String,
    data: Vec<Bank>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // pulling in environment vars defined in .env
    // should include CHAPA_API_PUBLIC_KEY
    dotenv().ok();

    const CHAPA_BASE_URL :&str = "https://api.chapa.co";
    let version = "v1";

    // NOTE: this operation is turbo-fished
    let api_key = env::var("CHAPA_API_PUBLIC_KEY")?;
    println!("{}", api_key);

    // headers hashmap
    let mut headers = HeaderMap::new();
    // this casting is necessary because since headers needs a HeaderValue not a string
    let api_key_header_value = format!("Bearer {}", api_key).parse().unwrap();
  //  let api_key_header_value = api_key.parse().unwrap();
    headers.insert(AUTHORIZATION, api_key_header_value);
   // headers.insert("apikey", api_key_header_value);

   println!("{}", format!("Bearer {}", api_key));
   // println!("{}", format!("{}/{}/banks", CHAPA_BASE_URL, version));

    // client
    let client = reqwest::Client::new();
    let banks_url = format!("{}/{}/banks", CHAPA_BASE_URL, version);
    let response = client.get(banks_url)
                    .headers(headers)
                    .send()
                    .await?;

   // println!("{:#?}", response);

    // deserizlization
    let response_json = response.json::<BankRequestResponse>().await?;
    println!("{:#?}", response_json);
    
    Ok(())
}