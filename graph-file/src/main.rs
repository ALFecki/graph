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

    return match OrientedGraph::<String, String>::deserialize(serialized_graph.as_str()) {
        Ok(graph) => {
            return match graph.depth_first_search(1) {
                Ok(dfs_result) => {
                    println!("{}", dfs_result);
                    Ok(())
                }
                Err(e) => {
                    Err(Box::new(e))
                }
            }
        }
        Err(e) => {
            Err(Box::new(e))
        }
    }
}
