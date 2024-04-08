pub mod serde_graph {
    use crate::edge::edge::DefaultEdge;
    use crate::error::GraphParseError;
    use crate::graph::graph::DefaultGraph;
    use crate::vertex::vertex::DefaultVertex;

    pub trait SerializeGraph<T> {
        fn serialize(graph: impl DefaultGraph<T>) -> String;
        fn serialize_vertex(vertex: impl DefaultVertex<T>) -> String;
        fn serialize_edge(edge: impl DefaultEdge<T>) -> String;
    }

    pub trait Deserialize<T> {
        fn deserialize(graph: &str) -> Result<impl DefaultGraph<T>, GraphParseError>;
        fn deserialize_vertex(vertex: &str) -> Result<impl DefaultVertex<T>, GraphParseError>;
        fn deserialize_edge(edge: &str) -> Result<impl DefaultEdge<T>, GraphParseError>;
    }
}
