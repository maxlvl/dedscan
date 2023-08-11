use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SubDomain {
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
