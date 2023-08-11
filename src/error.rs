use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: dedscan <example.com>")]
    CliUsage,
}
