pub mod edge {
    use std::rc::{Rc, Weak};
    use crate::graph::graph::DefaultGraph;
    use crate::vertex::vertex::DefaultVertex;

    pub trait DefaultEdge<T> {
        fn end() -> Weak<&dyn DefaultVertex<T>>;
    }

    pub trait DefaultBidirectionalEdge<T>: DefaultEdge<T> {
        fn start() ->  Weak<&dyn DefaultVertex<T>>;
    }

    pub struct Edge<T> {
        end: Weak<dyn DefaultVertex<T>>
    }

    impl<T> DefaultEdge<T> for Edge<T> {
        fn end() -> Weak<&dyn DefaultVertex<T>> {
            Self.end
        }
    }

    pub struct BidirectionalEdge<T> {
        start: Weak<dyn DefaultVertex<T>>,
        end: Weak<dyn DefaultVertex<T>>
    }


    impl<T> DefaultEdge<T> for BidirectionalEdge<T> {
        fn end() -> Weak<&dyn DefaultVertex<T>> {
            Self.end
        }
    }
    impl<T> DefaultBidirectionalEdge<T> for BidirectionalEdge<T> {
        fn start() ->  Weak<&dyn DefaultVertex<T>> {
            Self.start
        }
    }

}