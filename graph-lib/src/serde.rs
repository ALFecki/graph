pub mod serde_graph {
    pub trait SerializeGraph<T> {
        fn serialize(graph: dyn DefaultGraph<T>) -> str;
        fn serialize_vertex(vertex: dyn DefaultVertex<T>) -> str;
        fn serialize_edge(edge: dyn DefaultEdge<T>) -> str;
    }

    pub trait Deserialize<T> {
        fn deserialize(graph: str) -> dyn DefaultGraph<T>;
        fn deserialize_vertex(vertex: str) -> dyn DefaultVertex<T>;
        fn deserialize_edge(edge: str) -> dyn DefaultEdge<T>;
    }
}
