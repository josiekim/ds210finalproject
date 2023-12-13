//sixd.rs

use std::collections::{HashMap, HashSet, VecDeque};

pub struct Graph {
    pub edges: Vec<(i32, i32)>,
}

impl Graph {
    pub fn new(edges: Vec<(i32, i32)>) -> Self {
        Graph { edges }
    }

    // Auxiliary function to build the adjacency list
    fn build_adjacency_list(&self) -> HashMap<i32, Vec<i32>> {
        let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();

        for &(src, dst) in &self.edges {
            adjacency_list.entry(src).or_insert_with(Vec::new).push(dst);
            adjacency_list.entry(dst).or_insert_with(Vec::new).push(src);
        }

        adjacency_list
    }

    // Function to calculate six degrees of separation
    pub fn six_degrees_of_separation(&self, start_node: i32, target_node: i32) -> Option<u32> {
        let adjacency_list = self.build_adjacency_list();

        let mut visited: HashSet<i32> = HashSet::new();
        let mut queue: VecDeque<(i32, u32)> = VecDeque::new();

        queue.push_back((start_node, 0));

        while let Some((current_node, distance)) = queue.pop_front() {
            if current_node == target_node {
                return Some(distance);
            }

            if visited.contains(&current_node) {
                continue;
            }

            visited.insert(current_node);

            if let Some(neighbors) = adjacency_list.get(&current_node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        queue.push_back((neighbor, distance + 1));
                    }
                }
            }
        }

        None
    }

    // Function to calculate average degree of separation
    pub fn average_degree_of_separation(&self, node: i32) -> f64 {
        let adjacency_list = self.build_adjacency_list();

        let mut total_distance = 0;
        let mut total_paths = 0;

        let mut visited: HashSet<i32> = HashSet::new();
        let mut queue: VecDeque<(i32, u32)> = VecDeque::new();

        queue.push_back((node, 0));

        while let Some((current_node, distance)) = queue.pop_front() {
            if visited.contains(&current_node) {
                continue;
            }

            visited.insert(current_node);

            if distance > 0 {
                total_distance += distance;
                total_paths += 1;
            }

            if let Some(neighbors) = adjacency_list.get(&current_node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        queue.push_back((neighbor, distance + 1));
                    }
                }
            }
        }

        if total_paths > 1 {
            (((total_distance as f64) / ((total_paths - 1) as f64)) * 100.0).round() / 100.0
        } else {
            0.0
        }
    }

    // Function to calculate the mean degree of separation
    pub fn mean_degree_of_separation(&self) -> f64 {
        let total_degrees: u32 = (1..=self.edges.len() as i32)
            .map(|node| self.average_degree_of_separation(node))
            .map(|degree| degree as u32)
            .sum();

        let total_nodes = self.edges.len() as f64;
        if total_nodes > 0.0 {
            let mean_degree = total_degrees as f64 / total_nodes;
            (mean_degree * 100.0).round() / 100.0
        } else {
            0.0
        }
    }

    // Function to calculate the median degree of separation
    pub fn median_degree_of_separation(&self) -> f64 {
        let mut degrees: Vec<u32> = (1..=self.edges.len() as i32)
            .map(|node| self.average_degree_of_separation(node))
            .map(|degree| degree as u32)
            .collect();

        degrees.sort();

        if degrees.is_empty() {
            0.0
        } else if degrees.len() % 2 == 1 {
            degrees[degrees.len() / 2] as f64
        } else {
            let mid = degrees.len() / 2;
            let median = (degrees[mid - 1] + degrees[mid]) as f64 / 2.0;
            median
        }
    }
}
