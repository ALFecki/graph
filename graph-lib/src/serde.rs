pub mod serde_graph {
    use std::rc::Rc;
    use crate::edge::edge::DefaultEdge;
    use crate::error::GraphParseError;
    use crate::graph::graph::DefaultGraph;
    use crate::vertex::vertex::{DefaultVertex, Vertex};

    pub trait SerializeGraph<T, V> {
        fn serialize(graph: impl DefaultGraph<T, V>) -> String;
        fn serialize_vertex(vertex: impl DefaultVertex<T>) -> String;
        fn serialize_edge(edge: impl DefaultEdge<T>) -> String;
    }

    pub trait Deserialize<T, V> {
        fn deserialize(graph: &str) -> Result<impl DefaultGraph<T, V>, GraphParseError>;
        fn deserialize_vertex(vertex: &str) -> Result<impl DefaultVertex<T>, GraphParseError>;
        fn deserialize_edge(edge: &str, vertexes: Vec<Rc<Vertex<T, impl DefaultEdge<T>>>>) -> Result<impl DefaultEdge<T>, GraphParseError>;
    }
}
