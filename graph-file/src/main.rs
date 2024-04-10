use std::error::Error;
use std::fs::File;
use std::io::{stdin, Read};

use graph_lib::graph::graph::OrientedGraph;
use graph_lib::serde::serde_graph::DeserializeGraph;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter the filename: ");
    let mut filename = String::new();

    stdin().read_line(&mut filename)?;

    let mut file = File::open(filename.trim_end())?;

    let mut serialized_graph = String::new();

    file.read_to_string(&mut serialized_graph)?;

    let graph = OrientedGraph::<String, String>::deserialize(serialized_graph.as_str()).unwrap();

    let dfs_result = graph.depth_first_search(1).unwrap();
    println!("{}", dfs_result);

    Ok(())
}
