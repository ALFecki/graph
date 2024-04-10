#[cfg(test)]
mod tests {
    use super::*;
    use graph_lib::graph::graph::{DefaultGraph, OrientedGraph};
    use graph_lib::serde::serde_graph::{DeserializeGraph, SerializeGraph};

    const GRAPH_STR: &str = concat!(
        "1 First vertex\n",
        "2 Second vertex\n",
        "#\n",
        "1 2 Edge 1-2"
    );

    #[test]
    fn deserialization() {
        let res = OrientedGraph::<String, String>::deserialize(GRAPH_STR);
        assert!(res.is_ok());
    }

    #[test]
    fn vertexes_count() {
        let res = OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        assert_eq!(res.vertex_count(), 2);
    }

    #[test]
    fn edges_count() {
        let res = OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        assert_eq!(res.edges_count(), 1);
    }

    #[test]
    fn get_vertex_by_id() {
        let mut res = OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        assert!(res.get_vertex_by_id(1).is_some());
    }

    #[test]
    fn get_vertex_not_found() {
        let mut res = OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        assert!(res.get_vertex_by_id(5).is_none());
    }

    #[test]
    fn add_raw_vertex() {
        let mut res = OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        assert!(res.add_raw_vertex(4, "Test".to_string()).is_ok());
        assert!(res.add_raw_vertex(5, "Test1".to_string()).is_ok());
        
        assert_eq!(res.vertex_count(), 4);
    }

    #[test]
    fn add_edge_with_vertex_id() {
        let mut res = OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        assert!(res.add_raw_vertex(3, "Test".to_string()).is_ok());
        assert!(res.add_edge_with_vertex_id(3, 2, Some("Edge 3-2".to_string()))
            .is_ok());
        assert!(res.add_edge_with_vertex_id(2, 3, Some("Edge 2-3".to_string())).is_ok());
        assert_eq!(res.edges_count(), 3);
    }

    #[test]
    fn remove_edge() {
        let mut res = OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        assert!(res.add_raw_vertex(3, "Test".to_string()).is_ok());
        assert!(res.add_edge_with_vertex_id(3, 2, Some("Edge 3-2".to_string()))
            .is_ok());
        assert!(res.add_edge_with_vertex_id(2, 3, Some("Edge 2-3".to_string())).is_ok());

        assert_eq!(res.edges_count(), 3);
        assert!(res.remove_edge_by_vertexes(2, 3).is_ok());
        assert_eq!(res.edges_count(), 2);
    }
    
    #[test]
    fn remove_vertex() {
        let mut res = OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        assert_eq!(res.vertex_count(), 2);
        assert!(res.add_raw_vertex(3, "Test".to_string()).is_ok());
        assert_eq!(res.vertex_count(), 3);
        assert!(res.remove_vertex_by_id(3).is_ok());
        assert_eq!(res.vertex_count(), 2);
    }
    
    #[test]
    fn dfs_test() {
        let res = OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        let dfs_expect = concat!(
        "1 First vertex [2]\n",
        "2 Second vertex [1]\n",
        );
        let dfs_res = res.depth_first_search(1);
        assert!(dfs_res.is_ok());
        assert_eq!(dfs_expect, dfs_res.unwrap().to_string())
    }
    
    #[test]
    fn ser_test() {
        let res = OrientedGraph::<String, String>::deserialize(GRAPH_STR).unwrap();
        let ser_res = res.serialize();
        assert!(ser_res.is_ok());
        assert_eq!(GRAPH_STR, ser_res.unwrap());
    }
}
