pub mod serde_graph {
    use std::cell::{Ref, RefCell};
    use std::fmt::Debug;
    use std::rc::Rc;
    use std::str::FromStr;

    use crate::edge::edge::{DefaultEdge, DefaultOrientedEdge, OrientedEdge};
    use crate::error::{EdgeParseError, GraphParseError, SerializationError, VertexParseError};
    use crate::graph::graph::{DefaultGraph, OrientedGraph};
    use crate::vertex::vertex::{DefaultVertex, Vertex};

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

    impl<T: FromStr + Debug, V: FromStr + Debug + Clone> DeserializeGraph<T, V>
        for OrientedGraph<T, V>
    {
        type VertexType = Vertex<T, V>;
        type EdgeType = OrientedEdge<T, V>;
        type GraphType = OrientedGraph<T, V>;

        fn deserialize(graph: &str) -> Result<Self::GraphType, GraphParseError> {
            let mut graph_obj = Self::default();
            let mut deser_edges = false;
            for line in graph.lines() {
                if line.starts_with("#") {
                    deser_edges = true;
                    continue;
                }
                if deser_edges {
                    graph_obj
                        .add_edge(
                            Self::deserialize_edge(line, graph_obj.get_vertexes())
                                .map_err(|_| GraphParseError::EdgeParsingError)?,
                        )
                        .map_err(|_| GraphParseError::EdgeParsingError)?;
                } else {
                    graph_obj
                        .add_vertex(
                            Self::deserialize_vertex(line)
                                .map_err(|_| GraphParseError::VertexParsingError)?,
                        )
                        .map_err(|_| GraphParseError::VertexParsingError)?;
                }
            }
            Ok(graph_obj)
        }

        fn deserialize_vertex(vertex: &str) -> Result<Self::VertexType, VertexParseError> {
            if let Some((index, value)) = vertex.split_once(char::is_whitespace) {
                let vertex_id = index
                    .parse::<usize>()
                    .map_err(|_| VertexParseError::VertexIndexParsingError);
                let value = value
                    .parse::<T>()
                    .map_err(|_| VertexParseError::VertexValueParsingError);
                return Ok(Vertex::<T, V>::new(vertex_id?, value?));
            }
            Err(VertexParseError::VertexParsingError)
        }

        fn deserialize_edge(
            edge: &str,
            vertexes: Vec<Rc<RefCell<Self::VertexType>>>,
        ) -> Result<Self::EdgeType, EdgeParseError> {
            return if let Some((start, end_with_value)) = edge.split_once(char::is_whitespace) {
                let start_vertex = start
                    .parse::<usize>()
                    .map_err(|_| EdgeParseError::EdgeEndParsingError)
                    .and_then(|index| {
                        vertexes
                            .iter()
                            .find(|&p| p.borrow().id() == index)
                            .ok_or(EdgeParseError::VertexForEdgeIndexNotFound)
                    })?;

                let (end, value) = end_with_value
                    .split_once(char::is_whitespace)
                    .ok_or(EdgeParseError::EdgeParsingError)?;

                let end_vertex = end
                    .parse::<usize>()
                    .map_err(|_| EdgeParseError::EdgeStartParsingError)
                    .and_then(|index| {
                        vertexes
                            .iter()
                            .find(|&p| p.borrow().id() == index)
                            .ok_or(EdgeParseError::VertexForEdgeIndexNotFound)
                    })?;
                let value = value
                    .parse::<V>()
                    .map_err(|_| EdgeParseError::EdgeStartParsingError)?;
                Ok(OrientedEdge::<T, V>::new(start_vertex, end_vertex, value))
            } else {
                Err(EdgeParseError::EdgeParsingError)
            };
        }
    }

    impl<T: Debug + ToString, V: Debug + ToString + Clone> SerializeGraph<T, V>
        for OrientedGraph<T, V>
    {
        type VertexType = Vertex<T, V>;
        type EdgeType = OrientedEdge<T, V>;
        type GraphType = OrientedGraph<T, V>;

        fn serialize(&self) -> Result<String, SerializationError> {
            let mut result = String::new();
            for vertex in self.get_vertexes() {
                result.push_str(format!("{}\n", Self::serialize_vertex(vertex.borrow())).as_str());
            }
            result.push('#');
            for edge in self.get_edges() {
                match Self::serialize_edge(edge.borrow()) {
                    Ok(edge) => {
                        result.push_str(format!("\n{}", edge).as_str());
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Ok(result)
        }

        fn serialize_vertex(vertex: Ref<Self::VertexType>) -> String {
            String::from(format!(
                "{} {}",
                vertex.id().to_string(),
                vertex.value().to_string().as_str()
            ))
        }

        fn serialize_edge(edge: Ref<Self::EdgeType>) -> Result<String, SerializationError> {
            if let (Some(start), Some(end)) = (edge.start(), edge.end()) {
                return Ok(String::from(format!(
                    "{} {} {}",
                    start.borrow().id(),
                    end.borrow().id(),
                    if let Some(val) = edge.value() {
                        val.to_string()
                    } else {
                        "".to_string()
                    }
                )));
            }
            Err(SerializationError::EdgeVertexNotFound)
        }
    }
}
