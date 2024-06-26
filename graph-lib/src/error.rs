use thiserror::Error;

#[derive(Error, Debug)]
pub enum VertexParseError {
    #[error("Failed to parse vertex")]
    VertexParsingError,
    #[error("Failed to parse vertex index")]
    VertexIndexParsingError,
    #[error("Failed to parse vertex value")]
    VertexValueParsingError,
}

#[derive(Error, Debug)]
pub enum EdgeParseError {
    #[error("Failed to parse vertex")]
    EdgeParsingError,
    #[error("Failed to parse edge start")]
    EdgeStartParsingError,
    #[error("Failed to parse edge end")]
    EdgeEndParsingError,
    #[error("Vertex not found")]
    VertexForEdgeIndexNotFound,
    #[error("Failed to parse edge value")]
    EdgeValueParsingError,
}

#[derive(Error, Debug)]
pub enum GraphParseError {
    #[error("Failed to parse index")]
    VertexParsingError,
    #[error("Failed to parse value")]
    EdgeParsingError,
}

#[derive(Error, Debug)]
pub enum GraphError {
    #[error("Vertex is not found")]
    VertexNotFound,
    #[error("Edge is not found")]
    EdgeNotFound,
    #[error("Removing edge failed")]
    EdgeRemovingError,
    #[error("Removing vertex failed")]
    VertexRemovingError,
    #[error("Edge already exists")]
    EdgeExistsError,
    #[error("Vertex already exists")]
    VertexExistsError,
}

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("Edge has no one of the vertexes")]
    EdgeVertexNotFound,
}
