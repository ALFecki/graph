use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphParseError {
    #[error("Failed to parse vertex")]
    VertexParsingError,
    #[error("Failed to parse vertex index")]
    VertexIndexParsingError,
    #[error("Failed to parse vertex value")]
    VertexValueParsingError,
    #[error("Failed to parse edge")]
    EdgeParsingError
}