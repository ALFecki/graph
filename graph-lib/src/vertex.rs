pub mod vertex {
    use std::ops::Add;
    use std::rc::Rc;
    use crate::edge::edge::DefaultEdge;

    
    pub trait DefaultVertex<T> {
        type EdgeType: DefaultEdge<T>;

        // fn get_edges(&self) -> Vec<Self::EdgeType>;
        fn add_neighbor(&mut self, new_neighbor: Self::EdgeType);

        fn remove_neighbor(&self, vertex: impl DefaultVertex<T>) -> Result<(), ()>;

        fn get_id(&self) -> i32;
    }

    #[derive(PartialOrd, PartialEq)]
    pub struct Vertex<T, E: DefaultEdge<T>> {
        id: i32,
        value: T,
        edges: Vec<E>,
    }

    impl <T, E: DefaultEdge<T>> DefaultVertex<T> for Vertex<T, E> {
        type EdgeType = E;

        // fn get_edges(&self) -> Vec<Self::EdgeType> {
        //     self.edges
        // }

        fn add_neighbor(&mut self, new_neighbor: Self::EdgeType) {
            self.edges.push(new_neighbor)
        }

        fn remove_neighbor(&self, vertex: impl DefaultVertex<T>) -> Result<(), ()> {
            todo!()
        }

        fn get_id(&self) -> i32 {
            todo!()
        }
    }

}