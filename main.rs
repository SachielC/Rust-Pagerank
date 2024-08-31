use librs;

fn main() {
    let file_path = "data.txt";
    let graph = librs::read_graph::read_graph(file_path);
    let num_walks = 100;
    let pageranks = librs::approximate_pagerank::approximate_pagerank(&graph, num_walks);

    let mut sorted_pageranks: Vec<_> = pageranks.into_iter().collect();
    sorted_pageranks.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    for (_i, (vertex, rank)) in sorted_pageranks.iter().enumerate().take(5) {
        println!("vertex {}: approximate PageRank {:.4}", vertex, rank);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approximate_pagerank() {
        let mut graph = HashMap::new();
        graph.insert(0, vec![1]);
        graph.insert(1, vec![2]);
        graph.insert(2, vec![0]);

        let num_walks = 100;
        let pageranks = librs::approximate_pagerank::approximate_pagerank(&graph, num_walks);

        assert_eq!(pageranks.len(), 3);
        assert!(pageranks[&0] > 0.0);
        assert!(pageranks[&1] > 0.0);
        assert!(pageranks[&2] > 0.0);
    }
}
