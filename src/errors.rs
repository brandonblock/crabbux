/// An application-specific error type
#[derive(Debug)]
pub enum AccountingError {
    NotFound(String),
    UnderFunded(String, u64),
    OverFunded(String, u64),
}
