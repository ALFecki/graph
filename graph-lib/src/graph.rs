pub mod graph {
    use std::collections::HashSet;
    use std::rc::{Rc, Weak};
    use crate::edge::edge::{DefaultEdge, Edge, OrientedEdge};
    use crate::vertex::vertex::{DefaultVertex, Vertex};

    pub trait DefaultGraph<T> {
        fn vertex_count(&self) -> usize;
        fn edges_count(&self) -> usize;
        fn get_vertexes(&self) -> Vec<impl DefaultVertex<T>>;

        fn get_vertex_by_id(&self, id: usize) -> impl DefaultVertex<T>;

        fn add_edge(&mut self, edge: impl DefaultEdge<T>);
        fn add_edge_with_vertexes(&mut self, start: impl DefaultVertex<T>, end: impl DefaultVertex<T>);

    }

    pub struct OrientedGraph<T> {
        vertexes: Vec<Vertex<T, OrientedEdge<T, V>>>,
        // edges: Vec<OrientedEdge<T>>>>
    }

    impl<T> DefaultGraph<T> for OrientedGraph<T> {
        fn vertex_count(&self) -> usize {
            Self.vertexes.len()
        }

        fn edges_count(&self) -> usize {
            self.edges.len()
        }

        fn get_vertexes(&self) -> Vec<Vertex<T, OrientedEdge<T>>> {
            todo!()
        }

        fn get_vertex_by_id(&self, id: usize) -> Vertex<T, OrientedEdge<T>> {
            todo!()
        }

        fn add_edge(&mut self, edge: impl DefaultEdge<T>) {
            self.edges.push(edge)
        }

        fn add_edge_with_vertexes(&mut self, start: impl DefaultVertex<T>, end: impl DefaultVertex<T>) {
            self.edges.push(
                OrientedEdge::<T> {
                    start: Weak::new(start),
                    end: Weak::new(end)
                }
            )

        }
    }
}