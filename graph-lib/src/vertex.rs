pub mod vertex {
    use std::ops::Add;
    use std::rc::Rc;
    use crate::edge::edge::DefaultEdge;

    
    pub trait DefaultVertex<T> {
        type EdgeType: DefaultEdge<T>;

        // fn get_edges(&self) -> Vec<Self::EdgeType>;
        fn add_neighbor(&mut self, new_neighbor: Rc<Self::EdgeType>);
        fn find_neighbor(&self, vertex_id: usize);

        fn remove_neighbor(&self, vertex: impl DefaultVertex<T>) -> Result<(), ()>;

        fn get_id(&self) -> usize;
    }

    #[derive(PartialOrd, PartialEq)]
    pub struct Vertex<T, E: DefaultEdge<T>> {
        id: usize,
        value: T,
        edges: Vec<Rc<E>>,
    }
    
    impl<T, E: DefaultEdge<T>> Vertex<T, E> {
        pub(crate) fn new(id: usize, value: T) -> Self {
            Self {
                id,
                value,
                edges: Vec::default()
            }
        }
    }

    impl <T, E: DefaultEdge<T>> DefaultVertex<T> for Vertex<T, E> {
        type EdgeType = E;

        // fn get_edges(&self) -> Vec<Self::EdgeType> {
        //     self.edges
        // }

        fn add_neighbor(&mut self, new_neighbor: Rc<Self::EdgeType>) {
            self.edges.push(new_neighbor)
        }

        fn find_neighbor(&self, vertex_id: usize) {
            self.edges.iter().find(|&&p| if let Some(end) = p.end() {end.get_id() == vertex_id} else {})
        }


        fn remove_neighbor(&self, vertex: impl DefaultVertex<T>) -> Result<(), ()> {
            todo!()
        }

        fn get_id(&self) -> usize {
            self.id
        }
    }

}