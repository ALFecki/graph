pub mod edge {
    use std::cell::RefCell;
    use std::fmt::Debug;
    use std::ptr::null;
    use std::rc::{Rc, Weak};
    use crate::graph::graph::DefaultGraph;
    use crate::vertex::vertex::{DefaultVertex, Vertex};

    pub trait DefaultEdge<T> {
        type VertexType: DefaultVertex<T>;
        fn end(&self) -> Option<Rc<RefCell<Self::VertexType>>>;
        fn set_end(&mut self, vertex: &Rc<RefCell<Self::VertexType>>);
    }

    pub trait DefaultOrientedEdge<T>: DefaultEdge<T> {
        fn start(&self) -> Option<Rc<RefCell<Self::VertexType>>>;
        fn set_start(&mut self, vertex: &Rc<RefCell<Self::VertexType>>);
    }
    #[derive(Debug)]
    pub struct OrientedEdge<T: Debug, V: Debug> {
        start: Weak<RefCell<Vertex<T, V>>>,
        end: Weak<RefCell<Vertex<T, V>>>,
        value: Option<V>
    }

    impl<T: Debug, V: Debug> Default for OrientedEdge<T, V> {
        fn default() -> Self {
            Self {
                start: Weak::default(),
                end: Weak::default(),
                value: None
            }
        }
    }

    impl<T: Debug, V: Debug> OrientedEdge<T, V> {
        pub(crate) fn new(start: &Rc<RefCell<Vertex<T, V>>>, end: &Rc<RefCell<Vertex<T, V>>>, value: V) -> Self {
            Self {
                start: Rc::downgrade(&start),
                end: Rc::downgrade(&end),
                value: Some(value)
            }
        }
    }
    
    impl<T: Debug, V: Debug> DefaultEdge<T> for OrientedEdge<T, V> {
        type VertexType = Vertex<T, V>;

        fn end(&self) -> Option<Rc<RefCell<Self::VertexType>>> {
            self.end.upgrade()
        }

        fn set_end(&mut self, vertex: &Rc<RefCell<Self::VertexType>>) {
            self.end = Rc::downgrade(vertex)
        }
    }

    impl<T: Debug, V: Debug> DefaultOrientedEdge<T> for OrientedEdge<T, V> {
        fn start(&self) -> Option<Rc<RefCell<Self::VertexType>>> {
            self.start.upgrade()
        }

        fn set_start(&mut self, vertex: &Rc<RefCell<Self::VertexType>>) {
            self.start = Rc::downgrade(vertex);
        }
    }
}