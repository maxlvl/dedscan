// TODO:

// create public function called enumerate
// call crt.sh's api endpoint and specify output to be json
// declare a CrtEntryResult struct and specify what it should be depending on the response
// filter and deduplicate the responses and pour that into a hashSet (TBD)

// when that is done - declare a SubDomain struct that contains the subdomain info and open_ports, then pour your existing
// subdomains into a Vec of that struct with a non-initialized open_ports (as you'll add that in the port scanner
use crate::{
    model::{CrtShEntry, SubDomain},
    ports::scan_ports,
    Error,
};
use reqwest::blocking::Client;
use std::{collections::HashSet, time::Duration};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

pub fn enumerate(http_client: &Client, target: &str) -> Result<Vec<SubDomain>, Box<dyn Error>> {
    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()?
        .json()?;

    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .flat_map(|entry| {
            entry
                .name_value
                .split('\n')
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|subdomain: &String| subdomain != target)
        .filter(|subdomain: &String| !subdomain.contains('*'))
        .collect();

    subdomains.insert(target.to_string());

    let subdomains: Vec<SubDomain> = subdomains
        .into_iter()
        .map(|domain| SubDomain {
            domain,
            open_ports: Vec::new(),
        })
        .filter(resolves)
        .collect();

    let subdomains: Vec<SubDomain> = subdomains
        .into_iter()
        .map(|domain| scan_ports(domain))
        .collect();

    Ok(subdomains)
}

pub fn resolves(domain: &SubDomain) -> bool {
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(4);

    let dns_resolver = Resolver::new(ResolverConfig::default(), opts)
        .expect("subdomain resolver: building DNS client");
    dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}
