// main.rs

use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;
mod sixd;
mod tests;

fn main() {
    let file = File::open("euroroad.csv").unwrap();
    let reader = BufReader::new(file);

    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        // Ignore comment lines
        if line.starts_with("#") {
            continue;
        }

        let numbers: Vec<_> = line
            .split(",")
            .map(|s| i32::from_str_radix(s, 10))
            .filter_map(Result::ok)
            .collect();

        if numbers.len() == 2 {
            let (src, dst) = (numbers[0], numbers[1]);
            edges.push((src, dst));
        }
    }

    let graph = sixd::Graph::new(edges);
    let mut rng = rand::thread_rng();
    let random_node1: i32 = rng.gen_range(1..=100); 
    let random_node2: i32= rng.gen_range(1..=100);

    // Calculate and print degree between 2 random nodes
    if let Some(distance) = graph.six_degrees_of_separation(random_node1, random_node2) {
        println!(
            "The distance between nodes {} and {} is: {}",
            random_node1, random_node2, distance
        );
    } 
    else {
        println!("Nodes are not connected within six degrees of separation.");
    }

    // Calculate and print average degree of separation for specific nodes
    let average_degree_node_2 = graph.average_degree_of_separation(2);
    println!("Average degree of separation for node 2: {:.2}", average_degree_node_2);

    // Calculate and print mean degree of separation
    let mean_degree = graph.mean_degree_of_separation();
    println!("Mean degree of separation: {:.2}", mean_degree);

    // Calculate and print median degree of separation
    let median_degree = graph.median_degree_of_separation();
    println!("Median degree of separation: {:.2}", median_degree);

}
