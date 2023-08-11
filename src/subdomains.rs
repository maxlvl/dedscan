// TODO:

// create public function called enumerate
// call crt.sh's api endpoint and specify output to be json
// declare a CrtEntryResult struct and specify what it should be depending on the response
// filter and deduplicate the responses and pour that into a hashSet (TBD)

// when that is done - declare a SubDomain struct that contains the subdomain info and open_ports, then pour your existing
// subdomains into a Vec of that struct with a non-initialized open_ports (as you'll add that in the port scanner
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Subdomain {
    pub domain: String,
    pub open_ports: Vec<Port>,
}

#[derive(Debug, Clone)]
pub struct Port {
    pub port: u16,
    pub is_open: bool
}


#[derive(Debug, Deserialize, Serialize)]
pub struct CrtShEntry {
    pub issuer_ca_id: i32,
    pub issuer_name: Option<String>,
    pub common_name: Option<String>,
    pub name_value: Option<String>,
    pub id: i64,
    pub entry_timestamp: Option<String>,
    pub not_before: Option<String>,
    pub not_after: Option<String>,
    pub serial_number: Option<String>,
}

pub fn enumerate(http_client: &Client, target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()?
        .json()?;

    println!("{:#?}", entries);

    Ok(())
}
