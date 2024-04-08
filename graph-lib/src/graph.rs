pub mod graph {
    use std::collections::HashSet;
    use std::rc::{Rc, Weak};
    use crate::edge::edge::{DefaultEdge, Edge, OrientedEdge};
    use crate::vertex::vertex::{DefaultVertex, Vertex};

    pub trait DefaultGraph<T> {
        fn vertex_count() -> usize;
        fn edges_count() -> usize;
        fn get_vertexes() -> Vec<Rc<impl DefaultVertex<T>>>;

        fn get_vertex_by_id(id: usize) -> impl DefaultVertex<T>;

        fn add_edge(edge: Rc<impl DefaultEdge<T>>);
        fn add_edge_with_vertexes(start: Rc<impl DefaultVertex<T>>, end: Rc<impl DefaultVertex<T>>);

    }

    pub struct OrientedGraph<T> {
        vertexes: Vec<dyn DefaultVertex<T>>,
        edges: Vec<OrientedEdge<T>>
    }

    impl<T> DefaultGraph<T> for OrientedGraph<T> {
        fn vertex_count() -> usize {
            Self.vertexes.len()
        }

        fn edges_count() -> usize {
            Self.edges.len()
        }

        fn get_vertexes() -> Vec<Rc<dyn DefaultVertex<T>>> {
            todo!()
        }

        fn add_edge(edge: Rc<dyn DefaultEdge<T>>) {
            Self.edges.push(edge)
        }

        fn add_edge_with_vertexes(start: Rc<dyn DefaultVertex<T>>, end: Rc<dyn DefaultVertex<T>>) {
            Self.edges.push(
                OrientedEdge::<T> {
                    start: Weak::new(start),
                    end: Weak::new(end)
                }
            )

        }
    }
}