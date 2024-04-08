pub mod graph {
    use crate::edge::edge::{DefaultEdge, Edge, OrientedEdge};
    use crate::vertex::vertex::{DefaultVertex, Vertex};
    use std::collections::HashSet;
    use std::fmt::Error;
    use std::rc::{Rc, Weak};

    pub trait DefaultGraph<T> {
        type VertexType: DefaultVertex<T>;
        type EdgeType: DefaultEdge<T>;
        fn vertex_count(&self) -> usize;
        fn edges_count(&self) -> usize;
        fn get_vertexes(&self) -> Vec<Rc<Self::VertexType>>;

        fn get_vertex_by_id(&self, id: usize) -> Option<&Rc<Self::VertexType>>;

        fn add_edge(&mut self, edge: Self::EdgeType);
        fn add_edge_with_vertex_id(&mut self, start: usize, end: usize) -> Result<(), String>;
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
                start.add_neighbor(new_edge.clone());
                end.add_neighbor(new_edge.clone());
                self.edges.push(new_edge);
                Ok(())
            } else {
                Err(String::from("Cannot find vertexes"))
            }
        }
    }
}
