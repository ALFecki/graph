pub mod edge {
    use std::rc::{Rc, Weak};
    use crate::graph::graph::DefaultGraph;
    use crate::vertex::vertex::{DefaultVertex, Vertex};

    pub trait DefaultEdge<T> {
        type VertexType: DefaultVertex<T>;
        fn end(&self) -> Option<Rc<Self::VertexType>>;
    }

    pub trait DefaultOrientedEdge<T>: DefaultEdge<T> {
        fn start(&self) ->  Option<Rc<Self::VertexType>>;
    }

    pub struct Edge<T> {
        end: Weak<Vertex<T, Edge<T>>>
    }

    impl<T> DefaultEdge<T> for Edge<T> {
        type VertexType = Vertex<T, Edge<T>>;

        fn end(&self) -> Option<Rc<Self::VertexType>> {
            self.end.upgrade()
        }
    }

    pub struct OrientedEdge<T> {
        start: Weak<Vertex<T, OrientedEdge<T>>>,
        end:Weak<Vertex<T, OrientedEdge<T>>>
    }

    impl<T> OrientedEdge<T> {
        fn new(start: Rc<Vertex<T, OrientedEdge<T>>>, end: Rc<Vertex<T, OrientedEdge<T>>>) -> Self {
            Self {
                start: Rc::downgrade(&start),
                end: Rc::downgrade(&end)
            }
        }
    }


    impl<T> DefaultEdge<T> for OrientedEdge<T> {
        type VertexType = Vertex<T, OrientedEdge<T>>;

        fn end(&self) -> Option<Rc<Self::VertexType>> {
            self.end.upgrade()
        }
    }
    impl<T> DefaultOrientedEdge<T> for OrientedEdge<T> {
        fn start(&self) -> Option<Rc<Self::VertexType>> {
            self.start.upgrade()
        }
    }

}