pub mod vertex {
    use std::ops::Add;
    use std::rc::Rc;
    use crate::edge::edge::{DefaultEdge, OrientedEdge};

    
    pub trait DefaultVertex<T> {
        type EdgeType: DefaultEdge<T>;

        // fn get_edges(&self) -> Vec<Self::EdgeType>;
        fn add_neighbor(&mut self, new_neighbor: Rc<Self::EdgeType>);
        fn find_neighbor(&self, vertex_id: usize);

        fn remove_neighbor(&self, vertex: impl DefaultVertex<T>) -> Result<(), ()>;

        fn get_id(&self) -> usize;
    }

    // #[derive(PartialOrd, PartialEq)]
    pub struct Vertex<T, V> {
        id: usize,
        value: T,
        edges: Vec<Rc<OrientedEdge<T, V>>>,
    }
    
    impl<T, V> Vertex<T, V> {
        pub(crate) fn new(id: usize, value: T) -> Self {
            Self {
                id,
                value,
                edges: Vec::default()
            }
        }
    }

    impl <T, V> DefaultVertex<T> for Vertex<T, V> {
        type EdgeType = OrientedEdge<T, V>;

        // fn get_edges(&self) -> Vec<Self::EdgeType> {
        //     self.edges
        // }

        fn add_neighbor(&mut self, new_neighbor: Rc<Self::EdgeType>) {
            self.edges.push(new_neighbor)
        }

        fn find_neighbor(&self, vertex_id: usize) {
            // self.edges.iter().find(|&&p| if let Some(end) = p.end() {end.get_id() == vertex_id} else {})
        }


        fn remove_neighbor(&self, vertex: impl DefaultVertex<T>) -> Result<(), ()> {
            todo!()
        }

        fn get_id(&self) -> usize {
            self.id
        }
    }

}