// This module defines the Graph structure and implements
// graph algorithms such as BFS, centrality, and clustering.

use std::collections::{HashMap, HashSet, VecDeque};
use crate::data::Student;

const FEATURE_LABELS: [&str; 14] = [
    "age",
    "study_hours_per_day",
    "social_media_hours",
    "netflix_hours",
    "part_time_job",
    "attendance_percentage",
    "sleep_hours",
    "diet_quality",
    "exercise_frequency",
    "parental_education_level",
    "internet_quality",
    "mental_health_rating",
    "extracurricular_participation",
    "exam_score_placeholder"
];

/// Represents a graph where each node is a student,
/// and edges are formed based on similarity in feature space.
pub struct Graph {
    pub adjacency_list: HashMap<usize, Vec<usize>>,
    pub students: Vec<Student>,
}

impl Graph {
    /// Constructs a new Graph with an empty adjacency list and student data.
    pub fn new(students: Vec<Student>) -> Self {
        Graph {
            adjacency_list: HashMap::new(),
            students,
        }
    }

    /// Builds the similarity graph by connecting each student to their `k` most similar peers
    /// based on Euclidean distance over their feature vectors.
    pub fn build_similarity_graph(&mut self, k: usize) {
        for i in 0..self.students.len() {
            let mut similarities = Vec::new();
            for j in 0..self.students.len() {
                if i != j {
                    let sim = euclidean_distance(&self.students[i].features, &self.students[j].features);
                    similarities.push((j, sim));
                }
            }
            similarities.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let neighbors: Vec<usize> = similarities.into_iter().take(k).map(|(j, _)| j).collect();
            self.adjacency_list.insert(i, neighbors);
        }
    }

    /// Performs Breadth-First Search from `start` to `target` node.
    /// Returns the shortest path length if a path exists.
    pub fn bfs(&self, start: usize, target: usize) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        while let Some((node, dist)) = queue.pop_front() {
            if node == target {
                return Some(dist);
            }
            if visited.insert(node) {
                if let Some(neighbors) = self.adjacency_list.get(&node) {
                    for &neighbor in neighbors {
                        queue.push_back((neighbor, dist + 1));
                    }
                }
            }
        }
        None
    }

    /// Computes the degree distribution of all nodes in the graph.
    /// Returns a map of degree count -> number of students with that degree.
    pub fn degree_distribution(&self) -> HashMap<usize, usize> {
        let mut distribution = HashMap::new();
        for neighbors in self.adjacency_list.values() {
            let degree = neighbors.len();
            *distribution.entry(degree).or_insert(0) += 1;
        }
        distribution
    }

    /// Calculates closeness centrality for a given node.
    /// Closeness = (N - 1) / sum of shortest distances to all reachable nodes.
    pub fn closeness_centrality(&self, node: usize) -> f64 {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((node, 0));
        let mut total_distance = 0;

        while let Some((current, dist)) = queue.pop_front() {
            if visited.insert(current) {
                total_distance += dist;
                if let Some(neighbors) = self.adjacency_list.get(&current) {
                    for &neighbor in neighbors {
                        queue.push_back((neighbor, dist + 1));
                    }
                }
            }
        }

        if visited.len() <= 1 || total_distance == 0 {
            return 0.0;
        }
        (visited.len() as f64 - 1.0) / total_distance as f64
    }

    /// Identifies connected components in the graph using BFS.
    /// Each component is returned as a list of student indices.
    pub fn connected_components(&self) -> Vec<Vec<usize>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for &node in self.adjacency_list.keys() {
            if !visited.contains(&node) {
                let mut component = Vec::new();
                let mut queue = VecDeque::new();
                queue.push_back(node);
                visited.insert(node);

                while let Some(current) = queue.pop_front() {
                    component.push(current);
                    if let Some(neighbors) = self.adjacency_list.get(&current) {
                        for &neighbor in neighbors {
                            if visited.insert(neighbor) {
                                queue.push_back(neighbor);
                            }
                        }
                    }
                }

                components.push(component);
            }
        }

        components
    }

    /// Calculates the average score within each connected component (cluster).
    pub fn component_score_averages(&self, components: &[Vec<usize>]) -> Vec<f64> {
        components.iter().map(|component| {
            let total_score: f64 = component.iter().map(|&idx| self.students[idx].score).sum();
            total_score / component.len() as f64
        }).collect()
    }

    pub fn component_feature_averages(&self, components: &[Vec<usize>]) -> Vec<Vec<(String, f64)>> {
        let num_features = self.students[0].features.len();
        components.iter().map(|component| {
            let mut sums = vec![0.0; num_features];
            for &idx in component {
                for (i, val) in self.students[idx].features.iter().enumerate() {
                    sums[i] += val;
                }
            }
            sums.iter()
                .enumerate()
                .map(|(i, sum)| (FEATURE_LABELS[i].to_string(), sum / component.len() as f64))
                .collect()
        }).collect()
    }
}

// This helper function computes the Euclidean distance between two students' feature vectors.
pub fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Student;

    #[test]
    fn test_bfs_finds_path() {
        let students = vec![
            Student { features: vec![1.0], score: 50.0 },
            Student { features: vec![1.1], score: 60.0 },
            Student { features: vec![1.2], score: 70.0 },
        ];
        let mut graph = Graph::new(students);
        graph.build_similarity_graph(2); // ensure connections

        let path = graph.bfs(0, 2);
        assert!(path.is_some());
    }


    #[test]
    fn test_connected_components_multiple_groups() {
        let students = vec![
            Student { features: vec![0.0], score: 50.0 },
            Student { features: vec![0.1], score: 55.0 },
            Student { features: vec![5.0], score: 95.0 },
            Student { features: vec![5.1], score: 92.0 },
        ];
        let mut graph = Graph::new(students);
        graph.build_similarity_graph(1); // minimal links

        let components = graph.connected_components();
        assert!(components.len() >= 2);
    }
}
