use rayon::prelude::*;
use reqwest::blocking::Client;
use std::error::Error;

mod common_ports;
mod error;
mod model;
mod ports;
mod subdomains;

fn main() -> Result<(), Box<dyn Error>> {
    let http_client = Client::new();
    let target = "example.com";

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();
    
    pool.install(|| {
        let scan_result: Vec<SubDomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();
            // TODO continue this
    })

    let subdomains = subdomains::enumerate(&http_client, target);

    println!("{:#?}", subdomains);

    Ok(())
}
