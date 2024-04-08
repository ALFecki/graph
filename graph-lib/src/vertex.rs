pub mod vertex {
    use std::ops::Add;
    use std::rc::Rc;
    use crate::edge::edge::DefaultEdge;

    
    pub trait DefaultVertex<T> {

        fn get_edges() -> Vec<impl DefaultEdge<T>>;
        fn add_neighbor(new_neighbor: impl DefaultEdge<T>);

        fn remove_neighbor(vertex: impl DefaultVertex<T>) -> Result<(), ()>;

        fn get_id() -> i32;
    }

    #[derive(PartialOrd, PartialEq)]
    pub struct Vertex<T, E: DefaultEdge<T>> {
        id: i32,
        value: T,
        edges: Vec<E>,
    }

    impl <T, E: DefaultEdge<T>> DefaultVertex<T> for Vertex<T, E> {
        fn get_edges() -> Vec<E> {
            Self.edges
        }

        fn add_neighbor(new_neighbor: impl DefaultEdge<T>) {
            Self.edges.push(new_neighbor)
        }
 
        fn remove_neighbor(vertex: impl DefaultVertex<T>) -> Result<(), ()>{
            todo!()
        }

        fn get_id() -> i32 {
            Self.id
        }
    }

}