pub mod graph {
    use std::cell::{Ref, RefCell, RefMut};
    use std::fmt::{format, Debug};
    use std::ops::Deref;
    use std::rc::Rc;
    use std::str::FromStr;

    use crate::edge::edge::{DefaultEdge, DefaultOrientedEdge, OrientedEdge};
    use crate::error::{
        EdgeParseError, GraphError, GraphParseError, SerializationError, VertexParseError,
    };
    use crate::serde::serde_graph::{DeserializeGraph, SerializeGraph};
    use crate::vertex::vertex::{DefaultVertex, Vertex};

    pub trait DefaultGraph<T, V> {
        type VertexType: DefaultVertex<T, V>;
        type EdgeType: DefaultEdge<T, V>;
        fn vertex_count(&self) -> usize;
        fn edges_count(&self) -> usize;
        fn get_vertexes(&self) -> Vec<Rc<RefCell<Self::VertexType>>>;
        fn get_edges(&self) -> Vec<Rc<RefCell<Self::EdgeType>>>;
        fn get_vertex_by_id(&mut self, id: usize) -> Option<Rc<RefCell<Self::VertexType>>>;
        fn add_edge(&mut self, edge: Self::EdgeType);
        fn add_edge_with_vertex_id(
            &mut self,
            start: usize,
            end: usize,
            value: V,
        ) -> Result<(), GraphError>;

        fn remove_edge(&mut self, edge: &Rc<RefCell<Self::EdgeType>>) -> Result<(), GraphError>;

        fn remove_edge_by_vertexes(
            &mut self,
            start_vertex_id: usize,
            end_vertex_id: usize,
        ) -> Result<(), GraphError>;

        fn add_vertex(&mut self, vertex: Self::VertexType);
        fn add_raw_vertex(&mut self, id: usize, value: T);
        fn remove_vertex_by_id(&mut self, id: usize) -> Result<(), GraphError>;
        fn remove_edge_by_vertex_id(&mut self, start: usize, end: usize) -> Result<(), GraphError>;
    }

    #[derive(Debug)]
    pub struct OrientedGraph<T: Debug, V: Debug> {
        vertexes: Vec<Rc<RefCell<Vertex<T, V>>>>,
        edges: Vec<Rc<RefCell<OrientedEdge<T, V>>>>,
    }

    impl<T: Debug, V: Debug> Default for OrientedGraph<T, V> {
        fn default() -> Self {
            Self {
                vertexes: Vec::new(),
                edges: Vec::new(),
            }
        }
    }

    impl<T: Debug, V: Debug> DefaultGraph<T, V> for OrientedGraph<T, V> {
        type VertexType = Vertex<T, V>;
        type EdgeType = OrientedEdge<T, V>;

        fn vertex_count(&self) -> usize {
            self.vertexes.len()
        }

        fn edges_count(&self) -> usize {
            self.edges.len()
        }

        fn get_vertexes(&self) -> Vec<Rc<RefCell<Self::VertexType>>> {
            self.vertexes.clone()
        }

        fn get_edges(&self) -> Vec<Rc<RefCell<Self::EdgeType>>> {
            self.edges.clone()
        }

        fn get_vertex_by_id(&mut self, id: usize) -> Option<Rc<RefCell<Self::VertexType>>> {
            self.vertexes
                .iter()
                .find(|&p| p.borrow().id() == id)
                .cloned()
        }

        fn add_edge(&mut self, edge: Self::EdgeType) {
            self.edges.push(Rc::new(RefCell::new(edge)))
        }

        fn add_edge_with_vertex_id(
            &mut self,
            start: usize,
            end: usize,
            value: V,
        ) -> Result<(), GraphError> {
            let edge = Rc::new(RefCell::new(OrientedEdge::<T, V>::new_with_value(value)));

            if let Some(start) = self.get_vertex_by_id(start) {
                edge.borrow_mut().set_start(&start);
                start.borrow_mut().add_neighbor(edge.clone());
            }

            if let Some(end) = self.get_vertex_by_id(end) {
                edge.borrow_mut().set_end(&end);
                end.borrow_mut().add_neighbor(edge.clone())
            }

            self.edges.push(edge);
            Ok(())
        }

        fn remove_edge(
            &mut self,
            edge: &Rc<RefCell<OrientedEdge<T, V>>>,
        ) -> Result<(), GraphError> {
            self.edges.retain(|e| !Rc::ptr_eq(e, edge));

            if let (Some(start), Some(end)) = (edge.borrow().start(), edge.borrow().end()) {
                start
                    .borrow_mut()
                    .get_edges()
                    .retain(|e| !Rc::ptr_eq(&e, edge));
                end.borrow_mut()
                    .get_edges()
                    .retain(|e| !Rc::ptr_eq(&e, edge));
                return Ok(());
            }
            Err(GraphError::EdgeRemovingError)
        }
        fn remove_edge_by_vertexes(
            &mut self,
            start_vertex_id: usize,
            end_vertex_id: usize,
        ) -> Result<(), GraphError> {
            if let (Some(start), Some(end)) = (
                self.get_vertex_by_id(start_vertex_id),
                self.get_vertex_by_id(end_vertex_id),
            ) {
                let edge_index = start.borrow().get_edges().iter().position(|edge| {
                    edge.borrow().end().unwrap().borrow().id() == end.borrow().id()
                });

                if let Some(index) = edge_index {
                    let edge = start.borrow_mut().get_edges().remove(index);

                    self.edges.retain(|e| !Rc::ptr_eq(&e, &edge));
                    end.borrow_mut()
                        .get_edges()
                        .retain(|e| !Rc::ptr_eq(&e, &edge));

                    return Ok(());
                }
            }
            Err(GraphError::EdgeNotFound)
        }

        fn add_vertex(&mut self, vertex: Self::VertexType) {
            self.vertexes.push(Rc::new(RefCell::new(vertex)));
        }

        fn add_raw_vertex(&mut self, id: usize, value: T) {
            self.vertexes
                .push(Rc::new(RefCell::new(Vertex::<T, V>::new(id, value))))
        }

        fn remove_vertex_by_id(&mut self, id: usize) -> Result<(), GraphError> {
            let vertex_index = self
                .vertexes
                .iter()
                .position(|vertex| vertex.borrow().id() == id);

            if let Some(index) = vertex_index {
                let removed_vertex = self.vertexes.remove(index);

                self.edges.retain(|edge| {
                    let start_ptr = &edge.borrow().start();
                    let end_ptr = &edge.borrow().end();

                    start_ptr.clone().unwrap().borrow().id() != removed_vertex.clone().borrow().id()
                        && end_ptr.clone().unwrap().borrow().id()
                            != removed_vertex.clone().borrow().id()
                });

                for vertex in &self.vertexes {
                    let edges = &mut vertex.borrow_mut().get_edges();
                    edges.retain(|edge| {
                        let start_ptr = &edge.borrow().start();
                        let end_ptr = &edge.borrow().end();

                        start_ptr.clone().unwrap().borrow().id()
                            != removed_vertex.clone().borrow().id()
                            && end_ptr.clone().unwrap().borrow().id()
                                != removed_vertex.clone().borrow().id()
                    });
                }
                return Ok(());
            }
            Err(GraphError::VertexNotFound)
        }

        fn remove_edge_by_vertex_id(&mut self, start: usize, end: usize) -> Result<(), GraphError> {
            let vertex = self
                .get_vertex_by_id(start)
                .ok_or(GraphError::VertexNotFound)?;
            vertex.borrow_mut().remove_neighbor_by_position(end)?;
            Ok(())
        }
    }
}
