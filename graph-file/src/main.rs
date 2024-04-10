use graph_lib::graph::graph::{DefaultGraph, OrientedGraph};
use graph_lib::serde::serde_graph::{DeserializeGraph, SerializeGraph};

fn main() {
    let s = concat!(
        "1 First node\n",
        "2 Second node\n",
        "#\n",
        "1 2 Edge between the two\n"
    );

    let mut graph = OrientedGraph::<String, String>::deserialize(s).unwrap();
    println!("{:?}", graph);
    graph.add_raw_vertex(3, "Test".to_string());
    // graph
    //     .add_edge_with_vertex_id(1, 2, "Edge 1-2".to_string())
    //     .unwrap();
    graph
        .add_edge_with_vertex_id(2, 3, Some("Edge 2-3".to_string()))
        .unwrap();
    graph
        .add_edge_with_vertex_id(1, 3, Some("Edge 1-3".to_string()))
        .unwrap();
    graph
        .add_edge_with_vertex_id(3, 1, Some("Edge 3-1".to_string()))
        .unwrap();
    let result = graph.depth_first_search(1).unwrap();
    println!("{}", result);
    // graph.remove_vertex_by_id(2);

    let ser_graph = graph.serialize().unwrap();
    println!("{}", ser_graph)
}
