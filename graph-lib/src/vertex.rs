pub mod vertex {
    use std::cell::RefCell;
    use std::fmt::Debug;
    use std::rc::Rc;

    use crate::edge::edge::{DefaultEdge, DefaultOrientedEdge, OrientedEdge};

    pub trait DefaultVertex<T, V> {
        type EdgeType: DefaultEdge<T, V>;

        fn get_edges(&self) -> Vec<Rc<RefCell<Self::EdgeType>>>;
        fn add_neighbor(&mut self, new_neighbor: Rc<RefCell<Self::EdgeType>>);

        fn id(&self) -> usize;
        fn value(&self) -> &T;
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

        fn get_edges(&self) -> Vec<Rc<RefCell<Self::EdgeType>>> {
            self.edges.clone()
        }

        fn add_neighbor(&mut self, new_neighbor: Rc<RefCell<Self::EdgeType>>) {
            self.edges.push(new_neighbor)
        }

        fn id(&self) -> usize {
            self.id
        }

        fn value(&self) -> &T {
            &self.value
        }
    }
}
