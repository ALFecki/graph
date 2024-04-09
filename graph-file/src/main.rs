use graph_lib::graph::graph::OrientedGraph;
use graph_lib::serde::serde_graph::{DeserializeGraph, SerializeGraph};

fn main() {
    let s = concat!(
        "1 First node\n",
        "2 Second node\n",
        "#\n",
        "1 2 Edge between the two\n"
    );

    let graph = OrientedGraph::<String, String>::deserialize(s).unwrap();
    println!("{:?}", graph);
    
    let ser_graph = graph.serialize().unwrap();
    println!("{}", ser_graph)
}
