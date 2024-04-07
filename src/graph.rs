pub mod graph {
    use std::rc::Rc;
    use crate::vertex::vertex::DefaultVertex;

    pub trait DefaultGraph<T> {
        fn vertex_count() -> usize;
        fn edges_count();
        fn get_vertexes() -> Vec<Rc<dyn DefaultVertex<T>>>;
        fn add_edge(start: Rc<dyn DefaultVertex<T>>, end: Rc<dyn DefaultVertex<T>>);

    }

    pub trait OrientedDefault<T>: DefaultGraph<T> {
    }


    pub struct Graph<T> {
        vertexes: Vec<Rc<dyn DefaultVertex<T>>>
    }

    impl<T> DefaultGraph<T> for Graph<T> {
        fn vertex_count() -> usize {
            Self.vertexes.len()
        }

        fn edges_count() {
            todo!()
        }

        fn get_vertexes() -> Vec<Rc<dyn DefaultVertex<T>>> {
            Self.vertexes
        }

        fn add_edge(start: Rc<dyn DefaultVertex<T>>, end: Rc<dyn DefaultVertex<T>>) {
            todo!()
        }
    }
}