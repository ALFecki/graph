pub mod vertex {
    use crate::edge::edge::{DefaultEdge, DefaultOrientedEdge, OrientedEdge};
    use crate::error::GraphError;
    use std::cell::RefCell;
    use std::fmt::Debug;
    use std::ops::Add;
    use std::rc::Rc;

    pub trait DefaultVertex<T, V> {
        type EdgeType: DefaultEdge<T, V>;

        fn get_edges(&self) -> Vec<Rc<RefCell<Self::EdgeType>>>;
        fn add_neighbor(&mut self, new_neighbor: Rc<RefCell<Self::EdgeType>>);
        fn find_neighbor(&self, vertex_id: usize);

        fn remove_neighbor(&self, vertex: impl DefaultVertex<T, V>) -> Result<(), ()>;
        fn remove_neighbor_by_position(&mut self, remove_id: usize) -> Result<(), GraphError>;

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

        fn find_neighbor(&self, vertex_id: usize) {
            // self.edges.iter().find(|&&p| if let Some(end) = p.end() {end.get_id() == vertex_id} else {})
        }

        fn remove_neighbor(&self, vertex: impl DefaultVertex<T, V>) -> Result<(), ()> {
            todo!()
        }

        fn remove_neighbor_by_position(&mut self, remove_id: usize) -> Result<(), GraphError> {
            if let Some(pos) = self.edges.iter().position(|p| {
                let borrow = p.borrow();
                (borrow.start_id() == Some(self.id()) && borrow.end_id() == Some(remove_id))
                    || (borrow.start_id() == Some(remove_id) && borrow.end_id() == Some(self.id()))
            }) {
                self.edges.remove(pos);
                return Ok(());
            }
            Err(GraphError::EdgeNotFound)
        }

        fn id(&self) -> usize {
            self.id
        }

        fn value(&self) -> &T {
            &self.value
        }
    }
}
