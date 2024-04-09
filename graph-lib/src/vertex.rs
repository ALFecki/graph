pub mod vertex {
    use crate::edge::edge::{DefaultEdge, OrientedEdge};
    use std::cell::RefCell;
    use std::fmt::Debug;
    use std::ops::Add;
    use std::rc::Rc;

    pub trait DefaultVertex<T, V> {
        type EdgeType: DefaultEdge<T, V>;

        // fn get_edges(&self) -> Vec<Self::EdgeType>;
        fn add_neighbor(&mut self, new_neighbor: Rc<RefCell<Self::EdgeType>>);
        fn find_neighbor(&self, vertex_id: usize);

        fn remove_neighbor(&self, vertex: impl DefaultVertex<T, V>) -> Result<(), ()>;

        fn get_id(&self) -> usize;
    }

    #[derive(Debug, Clone)]
    pub struct Vertex<T: Debug, V: Debug> {
        id: usize,
        value: T,
        edges: Vec<Rc<RefCell<OrientedEdge<T, V>>>>,
    }

    impl<T: Debug, V: Debug> Vertex<T, V> {
        pub(crate) fn new(id: usize, value: T) -> Self {
            Self {
                id,
                value,
                edges: Vec::default(),
            }
        }
    }

    impl<T: Debug, V: Debug> DefaultVertex<T, V> for Vertex<T, V> {
        type EdgeType = OrientedEdge<T, V>;

        // fn get_edges(&self) -> Vec<Self::EdgeType> {
        //     self.edges
        // }

        fn add_neighbor(&mut self, new_neighbor: Rc<RefCell<Self::EdgeType>>) {
            self.edges.push(new_neighbor)
        }

        fn find_neighbor(&self, vertex_id: usize) {
            // self.edges.iter().find(|&&p| if let Some(end) = p.end() {end.get_id() == vertex_id} else {})
        }

        fn remove_neighbor(&self, vertex: impl DefaultVertex<T, V>) -> Result<(), ()> {
            todo!()
        }

        fn get_id(&self) -> usize {
            self.id
        }
    }
}
