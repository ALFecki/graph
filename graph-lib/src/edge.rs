pub mod edge {
    use std::rc::{Rc, Weak};
    use crate::graph::graph::DefaultGraph;
    use crate::vertex::vertex::DefaultVertex;

    pub trait DefaultEdge<T> {
        fn end() -> Weak<impl DefaultVertex<T>>;
    }

    pub trait DefaultOrientedEdge<T>: DefaultEdge<T> {
        fn start() ->  Weak<impl DefaultVertex<T>>;
    }

    pub struct Edge<T> {
        end: Weak<dyn DefaultVertex<T>>
    }

    impl<T> DefaultEdge<T> for Edge<T> {
        fn end() -> Weak<dyn DefaultVertex<T>> {
            Self.end
        }
    }

    pub struct OrientedEdge<T> {
        start: Weak<dyn DefaultVertex<T>>,
        end: Weak<dyn DefaultVertex<T>>
    }


    impl<T> DefaultEdge<T> for OrientedEdge<T> {
        fn end() -> Weak<dyn DefaultVertex<T>> {
            Self.end
        }
    }
    impl<T> DefaultOrientedEdge<T> for OrientedEdge<T> {
        fn start() ->  Weak<dyn DefaultVertex<T>> {
            Self.start
        }
    }

}