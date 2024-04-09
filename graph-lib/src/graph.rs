pub mod graph {
    use std::rc::Rc;
    use std::str::FromStr;

    use crate::edge::edge::{DefaultEdge, OrientedEdge};
    use crate::error::GraphParseError;
    use crate::serde::serde_graph::Deserialize;
    use crate::vertex::vertex::{DefaultVertex, Vertex};

    pub trait DefaultGraph<T> {
        type VertexType: DefaultVertex<T>;
        type EdgeType: DefaultEdge<T>;
        fn vertex_count(&self) -> usize;
        fn edges_count(&self) -> usize;
        fn get_vertexes(&self) -> Vec<Rc<Self::VertexType>>;

        fn get_vertex_by_id(&self, id: usize) -> Option<&Rc<Self::VertexType>>;

        fn add_edge(&mut self, edge: Self::EdgeType);
        fn add_edge_with_vertex_id(&mut self, start: usize, end: usize) -> Result<(), String>;

        fn add_vertex(&mut self, id: usize, value: T);
    }

    pub struct OrientedGraph<T> {
        vertexes: Vec<Rc<Vertex<T, OrientedEdge<T>>>>,
        edges: Vec<Rc<OrientedEdge<T>>>,
    }

    impl<T> DefaultGraph<T> for OrientedGraph<T> {
        type VertexType = Vertex<T, OrientedEdge<T>>;
        type EdgeType = OrientedEdge<T>;

        fn vertex_count(&self) -> usize {
            self.vertexes.len()
        }

        fn edges_count(&self) -> usize {
            self.edges.len()
        }

        fn get_vertexes(&self) -> Vec<Rc<Self::VertexType>> {
            self.vertexes.clone()
        }

        fn get_vertex_by_id(&self, id: usize) -> Option<&Rc<Self::VertexType>> {
            self.vertexes.iter().find(|&&p| p.get_id() == id)
        }

        fn add_edge(&mut self, edge: Self::EdgeType) {
            self.edges.push(Rc::new(edge))
        }

        fn add_edge_with_vertex_id(&mut self, start: usize, end: usize) -> Result<(), String> {
            if let (Some(mut start), Some(mut end)) =
                (self.get_vertex_by_id(start), self.get_vertex_by_id(end))
            {
                let new_edge = Rc::new(OrientedEdge::<T>::new(start.clone(), end.clone()));
                start.add_neighbor(Rc::clone(&new_edge));
                end.add_neighbor(Rc::clone(&new_edge));
                self.edges.push(new_edge);
                Ok(())
            } else {
                Err(String::from("Cannot find vertexes"))
            }
        }

        fn add_vertex(&mut self, id: usize, value: T) {
            self.vertexes.push(Rc::new(Vertex::<T, OrientedEdge<T>>::new(id, value)))
        }
    }

    impl<T: FromStr> Deserialize<T> for OrientedGraph<T> {
        fn deserialize(graph: &str) -> Result<OrientedGraph<T>, GraphParseError> {
            todo!()
        }

        fn deserialize_vertex(vertex: &str) -> Result<Vertex<T, OrientedEdge<T>>, GraphParseError> {
            if let Some((index, value)) = vertex.split_once(char::is_whitespace) {
                let vertex_id = index.parse::<usize>().map_err(|_| GraphParseError::VertexIndexParsingError);
                let value = index.parse::<T>().map_err(|_| GraphParseError::VertexValueParsingError);
                return Ok(Vertex::<T, OrientedEdge<T>>::new(vertex_id.unwrap(), value.unwrap()));
            }
            Err(GraphParseError::VertexParsingError)
        }

        fn deserialize_edge(edge: &str) -> Result<OrientedEdge<T>, GraphParseError> {
            
        }
    }
}
