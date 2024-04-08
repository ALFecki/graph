use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphParseError {
    #[error("Failed to parse vertex")]
    VertexParsingError,
    #[error("Failed to parse edge")]
    EdgeParsingError
}