pub mod graph {
    use std::cell::{RefCell, RefMut};
    use std::fmt::Debug;
    use std::ops::Deref;
    use std::rc::Rc;
    use std::str::FromStr;

    use crate::edge::edge::{DefaultEdge, DefaultOrientedEdge, OrientedEdge};
    use crate::error::{EdgeParseError, GraphParseError, VertexParseError};
    use crate::serde::serde_graph::Deserialize;
    use crate::vertex::vertex::{DefaultVertex, Vertex};

    pub trait DefaultGraph<T, V> {
        type VertexType: DefaultVertex<T>;
        type EdgeType: DefaultEdge<T>;
        fn vertex_count(&self) -> usize;
        fn edges_count(&self) -> usize;
        fn get_vertexes(&self) -> Rc<&Vec<Self::VertexType>>;

        fn get_vertex_by_id(&mut self, id: usize) -> Option<RefMut<&Self::VertexType>>;

        fn add_edge(&mut self, edge: Self::EdgeType);
        fn add_edge_with_vertex_id(
            &mut self,
            start: usize,
            end: usize,
            value: V,
        ) -> Result<(), String>;

        fn add_vertex(&mut self, vertex: Self::VertexType);

        fn add_raw_vertex(&mut self, id: usize, value: T);
    }

    #[derive(Debug)]
    pub struct OrientedGraph<T: Debug, V: Debug> {
        vertexes: Vec<Vertex<T, V>>,
        edges: Vec<OrientedEdge<T, V>>,
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

        fn get_vertexes(&self) -> Rc<&Vec<Self::VertexType>> {
            Rc::new(&self.vertexes)
        }

        fn get_vertex_by_id(&mut self, id: usize) -> Option<RefMut<&Self::VertexType>> {
             if let Some(founded) = self.vertexes.iter().find(|&p| p.get_id() == id) {
                 return Some(Rc::new(RefCell::new(founded)).borrow_mut());
            }
            None
        }

        fn add_edge(&mut self, edge: Self::EdgeType) {
            self.edges.push(edge)
        }

        fn add_edge_with_vertex_id(
            &mut self,
            start: usize,
            end: usize,
            value: V,
        ) -> Result<(), String> {
            let mut edge = Rc::new(OrientedEdge::<T, V>::default());
            let mut_edge = Rc::get_mut(&mut edge).ok_or("Cannot create edge mutable")?;

            if let Some(mut start) = self.get_vertex_by_id(start) {
                start.add_neighbor(edge);
                mut_edge.set_start(Rc::downgrade(start));
            }
            
            if let Some(mut end) = self.get_vertex_by_id(end) {
                end.add_neighbor(edge)
            }

            if let (Some(start), Some(end)) =
                (self.get_vertex_by_id(start), self.get_vertex_by_id(end))
            {
                let new_edge = Rc::new(OrientedEdge::<T, V>::new(
                    &start.clone(),
                    &end.clone(),
                    value,
                ));
                start.add_neighbor(Rc::clone(&new_edge));
                end.add_neighbor(Rc::clone(&new_edge));
                self.edges.push(new_edge);
                Ok(())
            } else {
                Err(String::from("Cannot find vertexes"))
            }
        }

        fn add_vertex(&mut self, vertex: Self::VertexType) {
            self.vertexes.push(vertex);
        }
        fn add_raw_vertex(&mut self, id: usize, value: T) {
            self.vertexes.push(Vertex::<T, V>::new(id, value))
        }
    }

    impl<T: FromStr + Debug, V: FromStr + Debug> Deserialize<T, V> for OrientedGraph<T, V> {

        fn deserialize(graph: &str) -> Result<OrientedGraph<T, V>, GraphParseError> {
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
                            .map_err(|_| GraphParseError::EdgeParsingError)?)
                } else {
                    graph_obj.add_vertex(Self::deserialize_vertex(line).map_err(|_| GraphParseError::VertexParsingError)?)
                }
            }
            Ok(graph_obj)
        }

        fn deserialize_vertex(vertex: &str) -> Result<Vertex<T, V>, VertexParseError> {
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
            vertexes: Vec<Rc<Vertex<T, V>>>,
        ) -> Result<OrientedEdge<T, V>, EdgeParseError> {
            return if let Some((end, start_with_value)) = edge.split_once(char::is_whitespace) {
                let end_vertex = end
                    .parse::<usize>()
                    .map_err(|_| EdgeParseError::EdgeEndParsingError)
                    .and_then(|index| {
                        vertexes
                            .iter()
                            .find(|&p| p.get_id() == index)
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
                            .find(|&p| p.get_id() == index)
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
