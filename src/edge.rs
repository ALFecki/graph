pub mod edge {
    use std::rc::Rc;
    use crate::vertex::vertex::DefaultVertex;

    pub trait DefaultEdge {
        fn end<T>() -> Rc<&dyn DefaultVertex<T>>;
    }
}