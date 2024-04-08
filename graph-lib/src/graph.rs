pub mod graph {
    use std::collections::HashSet;
    use std::rc::Rc;
    use crate::edge::edge::{DefaultEdge, Edge};
    use crate::vertex::vertex::{DefaultVertex, Vertex};

    pub trait DefaultGraph<T> {
        fn vertex_count() -> usize;
        fn edges_count() -> usize;
        fn get_vertexes() -> Vec<Rc<dyn DefaultVertex<T>>>;

        fn get_vertex_by_id(id: usize) -> dyn DefaultVertex<T>;

        fn add_edge(edge: Rc<dyn DefaultEdge<T>>);
        fn add_edge_with_vertexes(start: Rc<dyn DefaultVertex<T>>, end: Rc<dyn DefaultVertex<T>>);

    }

    pub struct Graph<T> {
        vertexes: HashSet<Rc<dyn DefaultVertex<T>>>,
        edges: Vec<Rc<Edge<T>>>
    }

    impl<T> DefaultGraph<T> for Graph<T> {
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


        }
    }
}