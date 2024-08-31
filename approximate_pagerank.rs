use rand::Rng;
use std::collections::HashMap;
use std::fs::File;


pub fn approximate_pagerank(graph: &HashMap<usize, Vec<usize>>, num_walks: usize) -> HashMap<usize, f64> {
    let mut pageranks: HashMap<usize, f64> = HashMap::new();

    for vertex in graph.keys() {
        pageranks.insert(*vertex, 0.0);
    }

    let n = graph.len() * num_walks;

    for _ in 0..num_walks {
        for vertex in graph.keys() {
            let mut rng = rand::thread_rng();
            let random_prob: f64 = rng.gen();

            if let Some(neighbors) = graph.get(vertex) {
                if neighbors.is_empty() {
                    *pageranks.get_mut(vertex).unwrap() += 1.0 / n as f64;
                } else if random_prob < 0.9 {
                    let random_neighbor = rng.gen_range(0..neighbors.len());
                    let next_vertex = neighbors[random_neighbor];
                    *pageranks.get_mut(&next_vertex).unwrap() += 1.0 / n as f64;
                } else {
                    let random_vertex = *graph.keys().nth(rng.gen_range(0..graph.len())).unwrap();
                    *pageranks.get_mut(&random_vertex).unwrap() += 1.0 / n as f64;
                }
            }
        }
    }

    pageranks
}