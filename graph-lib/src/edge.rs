pub mod edge {
    use std::rc::{Rc, Weak};
    use crate::graph::graph::DefaultGraph;
    use crate::vertex::vertex::{DefaultVertex, Vertex};

    pub trait DefaultEdge<T> {
        type VertexType: DefaultVertex<T>;
        fn end(&self) -> Option<Rc<Self::VertexType>>;
    }

    pub trait DefaultOrientedEdge<T>: DefaultEdge<T> {
        fn start(&self) -> Option<Rc<Self::VertexType>>;
    }

    pub struct OrientedEdge<T, V> {
        start: Weak<Vertex<T, OrientedEdge<T, V>>>,
        end: Weak<Vertex<T, OrientedEdge<T, V>>>,
        value: V
    }

    impl<T, V> OrientedEdge<T, V> {
        pub(crate) fn new(start: Rc<Vertex<T, OrientedEdge<T, V>>>, end: Rc<Vertex<T, OrientedEdge<T, V>>>, value: V) -> Self {
            Self {
                start: Rc::downgrade(&start),
                end: Rc::downgrade(&end),
                value
            }
        }
    }


    impl<T, V> DefaultEdge<T> for OrientedEdge<T, V> {
        type VertexType = Vertex<T, OrientedEdge<T, V>>;

        fn end(&self) -> Option<Rc<Self::VertexType>> {
            self.end.upgrade()
        }
    }

    impl<T, V> DefaultOrientedEdge<T> for OrientedEdge<T, V> {
        fn start(&self) -> Option<Rc<Self::VertexType>> {
            self.start.upgrade()
        }
    }
}