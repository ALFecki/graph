pub mod edge {
    use crate::graph::graph::DefaultGraph;
    use crate::vertex::vertex::{DefaultVertex, Vertex};
    use std::cell::RefCell;
    use std::fmt::Debug;
    use std::ptr::null;
    use std::rc::{Rc, Weak};

    pub trait DefaultEdge<T, V> {
        type VertexType: DefaultVertex<T, V>;
        fn end(&self) -> Option<Rc<RefCell<Self::VertexType>>>;
        fn end_id(&self) -> Option<usize>;
        fn set_end(&mut self, vertex: &Rc<RefCell<Self::VertexType>>);

        fn value(&self) -> Option<&V>;
        fn value_mut(&mut self) -> Option<&mut V>;
    }

    pub trait DefaultOrientedEdge<T, V>: DefaultEdge<T, V> {
        fn start(&self) -> Option<Rc<RefCell<Self::VertexType>>>;
        fn start_id(&self) -> Option<usize>;
        fn set_start(&mut self, vertex: &Rc<RefCell<Self::VertexType>>);
    }
    #[derive(Debug)]
    pub struct OrientedEdge<T: Debug, V: Debug> {
        start: Weak<RefCell<Vertex<T, V>>>,
        end: Weak<RefCell<Vertex<T, V>>>,
        value: Option<V>,
    }

    impl<T: Debug, V: Debug> Default for OrientedEdge<T, V> {
        fn default() -> Self {
            Self {
                start: Weak::default(),
                end: Weak::default(),
                value: None,
            }
        }
    }

    impl<T: Debug, V: Debug> OrientedEdge<T, V> {
        pub(crate) fn new(
            start: &Rc<RefCell<Vertex<T, V>>>,
            end: &Rc<RefCell<Vertex<T, V>>>,
            value: V,
        ) -> Self {
            Self {
                start: Rc::downgrade(&start),
                end: Rc::downgrade(&end),
                value: Some(value),
            }
        }

        pub(crate) fn new_with_value(value: V) -> Self {
            Self {
                value: Some(value),
                ..Self::default()
            }
        }
    }

    impl<T: Debug, V: Debug> DefaultEdge<T, V> for OrientedEdge<T, V> {
        type VertexType = Vertex<T, V>;

        fn end(&self) -> Option<Rc<RefCell<Self::VertexType>>> {
            self.end.upgrade()
        }

        fn end_id(&self) -> Option<usize> {
            self.end.upgrade().map(|val| val.borrow().id())
        }

        fn set_end(&mut self, vertex: &Rc<RefCell<Self::VertexType>>) {
            self.end = Rc::downgrade(vertex)
        }

        fn value(&self) -> Option<&V> {
            Option::from(&self.value)
        }

        fn value_mut(&mut self) -> Option<&mut V> {
            Option::from(&mut self.value)
        }
    }

    impl<T: Debug, V: Debug> DefaultOrientedEdge<T, V> for OrientedEdge<T, V> {
        fn start(&self) -> Option<Rc<RefCell<Self::VertexType>>> {
            self.start.upgrade()
        }

        fn start_id(&self) -> Option<usize> {
            self.start.upgrade().map(|val| val.borrow().id())
        }

        fn set_start(&mut self, vertex: &Rc<RefCell<Self::VertexType>>) {
            self.start = Rc::downgrade(vertex);
        }
    }
}
