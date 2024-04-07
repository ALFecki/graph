pub mod edge {
    use std::rc::Rc;
    use crate::graph::graph::DefaultGraph;
    use crate::vertex::vertex::DefaultVertex;

    pub trait DefaultEdge {
        fn end<T>() -> Rc<&dyn DefaultVertex<T>>;
    }

    pub trait DefaultBidirectionalEdge: DefaultEdge {
        fn start<T>() -> Rc<&dyn DefaultVertex<T>>;
    }

    pub struct Edge {

    }

}