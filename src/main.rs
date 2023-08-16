use reqwest::blocking::Client;
use std::error::Error;

mod error;
mod model;
mod subdomains;

fn main() -> Result<(), Box<dyn Error>> {
    let http_client = Client::new();
    let target = "example.com";

    let subdomains = subdomains::enumerate(&http_client, target);

    println!("{:#?}", subdomains);

    Ok(())
}
