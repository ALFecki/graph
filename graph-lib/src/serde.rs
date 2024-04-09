pub mod serde_graph {
    use crate::edge::edge::DefaultEdge;
    use crate::error::{EdgeParseError, GraphParseError, SerializationError, VertexParseError};
    use crate::graph::graph::DefaultGraph;
    use crate::vertex::vertex::{DefaultVertex, Vertex};
    use std::cell::{Ref, RefCell};
    use std::fmt::Debug;
    use std::rc::Rc;

    pub trait SerializeGraph<T, V> {
        type VertexType: DefaultVertex<T, V>;
        type EdgeType: DefaultEdge<T, V>;
        type GraphType: DefaultGraph<T, V>;
        fn serialize(&self) -> Result<String, SerializationError>;
        fn serialize_vertex(vertex: Ref<Self::VertexType>) -> String;
        fn serialize_edge(edge: Ref<Self::EdgeType>) -> Result<String, SerializationError>;
    }

    pub trait DeserializeGraph<T: Debug, V: Debug> {
        type VertexType: DefaultVertex<T, V>;
        type EdgeType: DefaultEdge<T, V>;
        type GraphType: DefaultGraph<T, V>;
        fn deserialize(graph: &str) -> Result<Self::GraphType, GraphParseError>;
        fn deserialize_vertex(vertex: &str) -> Result<Self::VertexType, VertexParseError>;
        fn deserialize_edge(
            edge: &str,
            vertexes: Vec<Rc<RefCell<Self::VertexType>>>,
        ) -> Result<Self::EdgeType, EdgeParseError>;
    }
}
