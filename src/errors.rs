/// An application-specific error type
#[derive(Debug)]
pub enum ApplicationError {
    NotFound(String),
    UnderFunded(String, u64),
    OverFunded(String, u64),
}
