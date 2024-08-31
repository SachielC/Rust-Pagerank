use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};


pub fn read_graph(file_path: &str) -> HashMap<usize, Vec<usize>> {
    let mut graph = HashMap::new();

    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        for (i, line) in reader.lines().enumerate() {
            if i == 0 {
                continue; // Skip the first line containing the number of vertices
            }

            if let Ok(edge) = line {
                let mut edge_parts = edge.split_whitespace().map(|s| s.parse::<usize>());

                if let (Some(Ok(src)), Some(Ok(dest))) = (edge_parts.next(), edge_parts.next()) {
                    graph.entry(src).or_insert(Vec::new()).push(dest);
                }
            }
        }
    }

    graph
}
