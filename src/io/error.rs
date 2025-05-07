#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Value is missing key {0}")]
    Missing(&'static str),
}
