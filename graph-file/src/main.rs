use graph_lib::graph::graph::OrientedGraph;
use graph_lib::serde::serde_graph::Deserialize;

fn main() {
    let s = concat!(
        "1 First node\n",
        "2 Second node\n",
        "#\n",
        "1 2 Edge between the two\n"
    );

    let graph = OrientedGraph::<String, String>::deserialize(s);
    println!("{:?}", graph)
}
