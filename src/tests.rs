#[cfg(test)]
mod tests {
    use crate::sixd::Graph;

    #[test]
    fn test_six_degrees_of_separation() {
        // Define a test graph
        let edges = vec![(1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7)];
        let graph = Graph::new(edges);

        // Test case: Nodes 1 and 5 are connected within six degrees of separation
        assert_eq!(graph.six_degrees_of_separation(1, 6), Some(5));
    }

    #[test]
    fn test_average_degree_of_separation() {
        // Define a test graph
        let edges = vec![(1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8)];
        let graph = Graph::new(edges);
    
        // Test case: Average degree of separation for node 1
        let avg_degree_node_1 = graph.average_degree_of_separation(1);
        assert_eq!(avg_degree_node_1, 4.67);
    
        // Test case: Average degree of separation for node 3
        let avg_degree_node_3 = graph.average_degree_of_separation(3);
        assert_eq!(avg_degree_node_3, 3.0);
    }

    #[test]
    fn test_mean_and_median_degree_of_separation() {
        // Define a test graph
        let edges = vec![(1, 2), (2, 3), (2, 4), (3, 5), (4, 6), (5, 7), (6, 7)];
        let graph = Graph::new(edges);

        // Test case: Mean and Median degree of separation
        let mean_degree = graph.mean_degree_of_separation();
        let median_degree = graph.median_degree_of_separation();

        assert_eq!(mean_degree, 2.14);
        assert_eq!(median_degree, 2.0);
    }

}