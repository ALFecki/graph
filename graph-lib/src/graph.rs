pub mod graph {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::fmt::{Debug, Display};
    use std::rc::Rc;

    use crate::edge::edge::{DefaultEdge, DefaultOrientedEdge, OrientedEdge};
    use crate::error::GraphError;
    use crate::vertex::vertex::{DefaultVertex, Vertex};

    pub trait DefaultGraph<T, V> {
        type VertexType: DefaultVertex<T, V>;
        type EdgeType: DefaultEdge<T, V>;
        fn vertex_count(&self) -> usize;
        fn edges_count(&self) -> usize;
        fn get_vertexes(&self) -> Vec<Rc<RefCell<Self::VertexType>>>;
        fn get_edges(&self) -> Vec<Rc<RefCell<Self::EdgeType>>>;
        fn get_vertex_by_id(&mut self, id: usize) -> Option<Rc<RefCell<Self::VertexType>>>;
        fn get_edge_by_vertexes_id(
            &self,
            start: usize,
            end: usize,
        ) -> Option<Rc<RefCell<Self::EdgeType>>>;
        fn add_edge(&mut self, edge: Self::EdgeType) -> Result<(), GraphError>;
        fn add_edge_with_vertex_id(
            &mut self,
            start: usize,
            end: usize,
            value: Option<V>,
        ) -> Result<(), GraphError>;

        fn remove_edge(&mut self, edge: &Rc<RefCell<Self::EdgeType>>) -> Result<(), GraphError>;

        fn remove_edge_by_vertexes(
            &mut self,
            start_vertex_id: usize,
            end_vertex_id: usize,
        ) -> Result<(), GraphError>;

        fn add_vertex(&mut self, vertex: Self::VertexType) -> Result<(), GraphError>;
        fn add_raw_vertex(&mut self, id: usize, value: T) -> Result<(), GraphError>;
        fn remove_vertex_by_id(&mut self, id: usize) -> Result<(), GraphError>;
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

    impl<T: Debug, V: Debug + Clone> DefaultGraph<T, V> for OrientedGraph<T, V> {
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

        fn get_edge_by_vertexes_id(
            &self,
            start: usize,
            end: usize,
        ) -> Option<Rc<RefCell<Self::EdgeType>>> {
            self.edges
                .iter()
                .find(|p| p.borrow().start_id() == Some(start) && p.borrow().end_id() == Some(end))
                .cloned()
        }

        fn add_edge(&mut self, edge: Self::EdgeType) -> Result<(), GraphError> {
            if let (Some(start), Some(end)) = (edge.start_id(), edge.end_id()) {
                if self.get_edge_by_vertexes_id(start, end).is_some() {
                    return Err(GraphError::EdgeExistsError);
                }
            }
            self.add_edge_with_vertex_id(
                edge.start_id().unwrap(),
                edge.end_id().unwrap(),
                edge.value().cloned(),
            )?;
            Ok(())
        }

        fn add_edge_with_vertex_id(
            &mut self,
            start: usize,
            end: usize,
            value: Option<V>,
        ) -> Result<(), GraphError> {
            if self.get_edge_by_vertexes_id(start, end).is_some() {
                return Err(GraphError::EdgeExistsError);
            }
            let edge = Rc::new(RefCell::new(OrientedEdge::<T, V>::new_with_value(value)));

            if let (Some(start), Some(end)) =
                (self.get_vertex_by_id(start), self.get_vertex_by_id(end))
            {
                edge.borrow_mut().set_start(&start);
                edge.borrow_mut().set_end(&end);

                start.borrow_mut().add_neighbor(edge.clone());
                end.borrow_mut().add_neighbor(edge.clone());
                self.edges.push(edge);
                return Ok(());
            }
            Err(GraphError::VertexNotFound)
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

        fn add_vertex(&mut self, vertex: Self::VertexType) -> Result<(), GraphError> {
            if self.get_vertex_by_id(vertex.id()).is_some() {
                return Err(GraphError::VertexExistsError);
            }

            self.vertexes.push(Rc::new(RefCell::new(vertex)));
            Ok(())
        }

        fn add_raw_vertex(&mut self, id: usize, value: T) -> Result<(), GraphError> {
            if self.get_vertex_by_id(id).is_some() {
                return Err(GraphError::VertexExistsError);
            }
            self.vertexes
                .push(Rc::new(RefCell::new(Vertex::<T, V>::new(id, value))));
            Ok(())
        }

        fn remove_vertex_by_id(&mut self, id: usize) -> Result<(), GraphError> {
            let vertex_index = self
                .vertexes
                .iter()
                .position(|vertex| vertex.borrow().id() == id);

            if let Some(index) = vertex_index {
                let removed_vertex = self.vertexes.remove(index);

                self.edges.retain(|edge| {
                    if let (Some(start_ptr), Some(end_ptr)) =
                        (&edge.borrow().start(), &edge.borrow().end())
                    {
                        return start_ptr.borrow().id() != removed_vertex.clone().borrow().id()
                            && end_ptr.borrow().id() != removed_vertex.clone().borrow().id();
                    };
                    true
                });

                for vertex in &self.vertexes {
                    let edges = &mut vertex.borrow_mut().get_edges();
                    edges.retain(|edge| {
                        if let (Some(start_ptr), Some(end_ptr)) =
                            (&edge.borrow().start(), &edge.borrow().end())
                        {
                            return start_ptr.borrow().id() != removed_vertex.clone().borrow().id()
                                && end_ptr.borrow().id() != removed_vertex.clone().borrow().id();
                        }
                        true
                    });
                }
                return Ok(());
            }
            Err(GraphError::VertexNotFound)
        }
    }

    pub struct DFSResult<T: Debug, V: Debug>(Vec<Rc<RefCell<Vertex<T, V>>>>);

    impl<T: Debug + ToString, V: Debug + ToString> Display for DFSResult<T, V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut res = String::new();
            for vertex in self.0.clone() {
                let borrow = vertex.borrow();
                let mut adjacent: Vec<usize> = borrow
                    .get_edges()
                    .iter()
                    .filter_map(|p| {
                        let edge_borrow = p.borrow();
                        if edge_borrow.start_id() != Some(borrow.id()) {
                            edge_borrow.start_id()
                        } else {
                            edge_borrow.end_id()
                        }
                    })
                    .collect();
                adjacent.dedup_by(|l, r| l == r);
                res.push_str(
                    format!(
                        "{} {} {:?}\n",
                        borrow.id(),
                        borrow.value().to_string(),
                        adjacent
                    )
                    .as_str(),
                );
            }
            write!(f, "{}", res)
        }
    }

    impl<T: Debug, V: Debug> OrientedGraph<T, V> {
        pub fn depth_first_search(
            &self,
            start_vertex_id: usize,
        ) -> Result<DFSResult<T, V>, GraphError> {
            let mut result = Vec::new();
            let start_vertex = self
                .vertexes
                .iter()
                .find(|vertex| vertex.borrow().id() == start_vertex_id);

            if let Some(vertex) = start_vertex {
                let mut visited: HashMap<usize, bool> = HashMap::new();
                result.push(vertex.clone());
                self.dfs_helper(vertex, &mut visited, &mut result);
                return Ok(DFSResult::<T, V> { 0: result });
            }
            Err(GraphError::VertexNotFound)
        }

        fn dfs_helper(
            &self,
            vertex: &Rc<RefCell<Vertex<T, V>>>,
            visited: &mut HashMap<usize, bool>,
            result: &mut Vec<Rc<RefCell<Vertex<T, V>>>>,
        ) {
            visited.insert(vertex.borrow().id(), true);

            for edge in &vertex.borrow().get_edges() {
                let neighbor = edge.borrow().end();
                if let Some(neighbor) = neighbor {
                    let neighbor_id = neighbor.borrow().id();
                    if visited.get(&neighbor_id).is_none() {
                        result.push(neighbor.clone());
                        self.dfs_helper(&neighbor, visited, result);
                    }
                }
            }
        }
    }
}
