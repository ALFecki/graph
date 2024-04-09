#[cfg(test)]
mod tests {
    use graph_lib::graph::graph::{DefaultGraph, OrientedGraph};
    use graph_lib::serde::serde_graph::DeserializeGraph;
    use super::*;
    
    const GRAPH_STR: &str = concat!(
    "1 First node\n",
    "2 Second node\n",
    "#\n",
    "1 2 Edge between the two\n"
    );
    
    #[test]
    fn deserialization() {
        let res = OrientedGraph::<String, String>::deserialize(GRAPH_STR);
        assert!(res.is_ok());
    }
    
    #[test]
    fn vertexes_count() {
        let res =OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        assert_eq!(res.vertex_count(), 2);
    }
    
    #[test]
    fn edges_count() {
        let res =OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        assert_eq!(res.edges_count(), 1);
    }
    
    #[test]
    fn get_vertex_by_id() {
        let mut res =OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        assert!(res.get_vertex_by_id(1).is_some());
    }
    
    #[test]
    fn get_vertex_not_found() {
        let mut res =OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        assert!(res.get_vertex_by_id(5).is_none());
    }
    
    #[test]
    fn add_raw_vertex() {
        let mut res =OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        res.add_raw_vertex(4, "Test".to_string());
        res.add_raw_vertex(5, "Test1".to_string());
        assert_eq!(res.vertex_count(), 4);
    }
    
    #[test]
    fn add_edge_with_vertex_id() {
        let mut res =OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        res.add_raw_vertex(3, "Test".to_string());
        res.add_edge_with_vertex_id(1, 2, "Edge 1-2".to_string()).unwrap();
        res.add_edge_with_vertex_id(2, 3, "Edge 2-3".to_string()).unwrap();
        assert_eq!(res.edges_count(), 3);
    }
    
    
    
}
