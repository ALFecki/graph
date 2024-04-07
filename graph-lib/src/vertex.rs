pub mod vertex {
    use std::ops::Add;
    use std::rc::Rc;
    use crate::edge::edge::DefaultEdge;

    pub trait DefaultVertex<T> {
        fn get_edges() -> Vec<Rc<dyn DefaultEdge>>;
        fn add_neighbor(new_neighbor: Rc<dyn DefaultEdge>);

        fn remove_neighbor(vertex: Rc<dyn DefaultVertex<T>>) -> Result<Ok, Err>;

        fn get_id() -> i32;
    }

    #[derive(PartialOrd, PartialEq)]
    pub struct Vertex<T> {
        id: i32,
        value: T,
        edges: Vec<Rc<dyn DefaultEdge>>,

    }

    impl <T> DefaultVertex<T> for Vertex<T> {
        fn get_edges() -> Vec<Rc<dyn DefaultEdge>> {
            Self.edges
        }

        fn add_neighbor(new_neighbor: Rc<dyn DefaultEdge>){
            Self.edges.push(new_neighbor)
        }

        fn remove_neighbor(vertex: Rc<dyn DefaultVertex<T>>) -> Result<Ok, Err>{
            todo!()
        }

        fn get_id() -> i32 {
            Self.id
        }
    }

}