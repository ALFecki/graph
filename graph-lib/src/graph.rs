pub mod graph {
    use std::cell::{RefCell, RefMut};
    use std::fmt::Debug;
    use std::ops::Deref;
    use std::rc::Rc;
    use std::str::FromStr;

    use crate::edge::edge::{DefaultEdge, DefaultOrientedEdge, OrientedEdge};
    use crate::error::{EdgeParseError, GraphError, GraphParseError, VertexParseError};
    use crate::serde::serde_graph::Deserialize;
    use crate::vertex::vertex::{DefaultVertex, Vertex};

    pub trait DefaultGraph<T, V> {
        type VertexType: DefaultVertex<T, V>;
        type EdgeType: DefaultEdge<T, V>;
        fn vertex_count(&self) -> usize;
        fn edges_count(&self) -> usize;
        fn get_vertexes(&self) -> Vec<Rc<RefCell<Self::VertexType>>>;

        fn get_vertex_by_id(&mut self, id: usize) -> Option<Rc<RefCell<Self::VertexType>>>;

        fn add_edge(&mut self, edge: Self::EdgeType);
        fn add_edge_with_vertex_id(
            &mut self,
            start: usize,
            end: usize,
            value: V,
        ) -> Result<(), GraphError>;

        fn add_vertex(&mut self, vertex: Self::VertexType);

        fn add_raw_vertex(&mut self, id: usize, value: T);
    }

    #[derive(Debug)]
    pub struct OrientedGraph<T: Debug, V: Debug> {
        vertexes: Vec<Rc<RefCell<Vertex<T, V>>>>,
        edges: Vec<Rc<RefCell<OrientedEdge<T, V>>>>,
    }

    impl<T: Debug, V: Debug> Default for OrientedGraph<T, V> {
        fn default() -> Self {
            Self {
                vertexes: Vec::new(),
                edges: Vec::new(),
            }
        }
    }

    impl<T: Debug, V: Debug> DefaultGraph<T, V> for OrientedGraph<T, V> {
        type VertexType = Vertex<T, V>;
        type EdgeType = OrientedEdge<T, V>;

        fn vertex_count(&self) -> usize {
            self.vertexes.len()
        }

        fn edges_count(&self) -> usize {
            self.edges.len()
        }

        fn get_vertexes(&self) -> Vec<Rc<RefCell<Self::VertexType>>> {
            self.vertexes.clone()
        }

        fn get_vertex_by_id(&mut self, id: usize) -> Option<Rc<RefCell<Self::VertexType>>> {
            if let Some(founded) = self.vertexes.iter().find(|&p| p.borrow().get_id() == id) {
                return Some(founded.clone());
            }
            None
        }

        fn add_edge(&mut self, edge: Self::EdgeType) {
            self.edges.push(Rc::new(RefCell::new(edge)))
        }

        fn add_edge_with_vertex_id(
            &mut self,
            start: usize,
            end: usize,
            value: V,
        ) -> Result<(), GraphError> {
            let mut edge = Rc::new(RefCell::new(OrientedEdge::<T, V>::new_with_value(value)));

            if let Some(mut start) = self.get_vertex_by_id(start) {
                edge.borrow_mut().set_start(&start);
                start.borrow_mut().add_neighbor(edge.clone());
            }

            if let Some(mut end) = self.get_vertex_by_id(end) {
                edge.borrow_mut().set_end(&end);
                end.borrow_mut().add_neighbor(edge.clone())
            }
            
            self.edges.push(edge);
            Ok(())
        }

        fn add_vertex(&mut self, vertex: Self::VertexType) {
            self.vertexes.push(Rc::new(RefCell::new(vertex)));
        }
        fn add_raw_vertex(&mut self, id: usize, value: T) {
            self.vertexes
                .push(Rc::new(RefCell::new(Vertex::<T, V>::new(id, value))))
        }
    }

    impl<T: FromStr + Debug, V: FromStr + Debug> Deserialize<T, V> for OrientedGraph<T, V> {
        type VertexType = Vertex<T,V>;
        type EdgeType = OrientedEdge<T, V>;
        type GraphType = OrientedGraph<T,V>;

        fn deserialize(graph: &str) -> Result<Self::GraphType, GraphParseError> {
            let mut graph_obj = Self::default();
            let mut deser_edges = false;
            for line in graph.lines() {
                if line.starts_with("#") {
                    deser_edges = true;
                    continue;
                }
                if deser_edges {
                    graph_obj.add_edge(
                        Self::deserialize_edge(line, graph_obj.vertexes.clone())
                            .map_err(|_| GraphParseError::EdgeParsingError)?,
                    )
                } else {
                    graph_obj.add_vertex(
                        Self::deserialize_vertex(line)
                            .map_err(|_| GraphParseError::VertexParsingError)?,
                    )
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
            vertexes: Vec<Rc<RefCell<Self::VertexType>>>
        ) -> Result<Self::EdgeType, EdgeParseError> {
            return if let Some((end, start_with_value)) = edge.split_once(char::is_whitespace) {
                let end_vertex = end
                    .parse::<usize>()
                    .map_err(|_| EdgeParseError::EdgeEndParsingError)
                    .and_then(|index| {
                        vertexes
                            .iter()
                            .find(|&p| p.borrow().get_id() == index)
                            .ok_or(EdgeParseError::VertexForEdgeIndexNotFound)
                    })?;

                let (start, value) = start_with_value
                    .split_once(char::is_whitespace)
                    .ok_or(EdgeParseError::EdgeParsingError)?;

                let start_vertex = start
                    .parse::<usize>()
                    .map_err(|_| EdgeParseError::EdgeStartParsingError)
                    .and_then(|index| {
                        vertexes
                            .iter()
                            .find(|&p| p.borrow().get_id() == index)
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
}
