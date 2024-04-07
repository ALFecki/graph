pub mod vertex {
    use std::ops::Add;
    use crate::edge::edge::Edge;

    pub trait Vertex<T> {
        fn get_edges() -> &dyn Edge;
        fn add_neighbor();

        fn remove_neighbor() -> &dyn Vertex<T>;

        fn get_id() -> i32;
        fn swap();
    }

    impl<T> Add<&dyn Vertex<T>> for &dyn Vertex<T> {
        type Output = ();

        fn add(self, rhs: &dyn Vertex<T>) -> Self::Output {
            todo!()
        }
    }

}