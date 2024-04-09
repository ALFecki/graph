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
    graph.add_edge_with_vertex_id(1, 2, "Edge 1-2".to_string()).unwrap();
    graph.add_edge_with_vertex_id(2, 3, "Edge 2-3".to_string()).unwrap();
    graph.remove_edge_by_vertex_id(2, 3).unwrap();
    
    let ser_graph = graph.serialize().unwrap();
    println!("{}", ser_graph)
}
