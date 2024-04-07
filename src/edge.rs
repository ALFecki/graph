pub mod edge {
    use std::rc::Rc;
    use crate::vertex::vertex::Vertex;

    pub trait Edge {
        fn end<T>() -> Rc<&dyn Vertex<T>>;
    }
}