// TODO:

// create public function called enumerate
// call crt.sh's api endpoint and specify output to be json
// declare a CrtEntryResult struct and specify what it should be depending on the response
// filter and deduplicate the responses and pour that into a hashSet (TBD)

// when that is done - declare a SubDomain struct that contains the subdomain info and open_ports, then pour your existing
// subdomains into a Vec of that struct with a non-initialized open_ports (as you'll add that in the port scanner
use reqwest::blocking::Client;
// use std::collections::HashSet;
use crate::{
    model::{
        CrtShEntry,
        SubDomain,
    },
    Error,
};


pub fn enumerate(http_client: &Client, target: &str) -> Result<(), Box<dyn Error>> {
    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()?
        .json()?;

    println!("{:#?}", entries);

    Ok(())
}
