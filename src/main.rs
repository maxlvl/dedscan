use rayon::prelude::*;
use reqwest::blocking::Client;
use std::error::Error;

mod common_ports;
mod error;
mod model;
mod ports;
mod subdomains;

#[tokio::main]
fn main() -> Result<(), anyhow::Error> {
    let http_timeout = Duration::from_secs(10)
    let http_client = Client::new();
    let target = "10.10.38.228";

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(255)
        .build()
        .unwrap();

    println!(
        "Scanning with {} number of threads in the ThreadPool",
        pool.current_num_threads()
    );

    pool.install(|| {
        let scan_result: Vec<model::SubDomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();

        for subdomain in scan_result {
            println!("{}:", subdomain.domain);
            for port in subdomain.open_ports {
                println!("      {}", port.port)
            }
            println!("")
        }
    });

    Ok(())
}
