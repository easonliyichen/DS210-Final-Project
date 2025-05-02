mod graph;
mod data;

use graph::Graph;
use data::load_students_from_csv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let students = load_students_from_csv("student_habits_performance.csv")?;
    println!("Loaded {} students.\n", students.len());

    let mut graph = Graph::new(students);
    graph.build_similarity_graph(3); // Connect top-3 most similar

    println!("Graph constructed with {} nodes.\n", graph.adjacency_list.len());

    // Example: BFS between student 0 and 10
    if let Some(dist) = graph.bfs(0, 10) {
        println!("Shortest path from 0 to 10 is {} steps.\n", dist);
    } else {
        println!("No path between 0 and 10.\n");
    }

    // Print degree distribution
    let degrees = graph.degree_distribution();
    println!("Degree distribution:");
    for (degree, count) in degrees.iter() {
        println!("  Degree {}: {} students", degree, count);
    }
    println!();

    // Print closeness centrality for node 0
    let closeness = graph.closeness_centrality(0);
    println!("Closeness centrality of student 0: {:.4}\n", closeness);

    // Connected components analysis
    let components = graph.connected_components();
    println!("Found {} connected components.\n", components.len());

    // Average score and features per cluster with size
    let averages = graph.component_score_averages(&components);
    let features = graph.component_feature_averages(&components);
    let mut cluster_info: Vec<(usize, usize, f64, Vec<(String, f64)>)> = components
        .iter()
        .enumerate()
        .map(|(i, comp)| (i + 1, comp.len(), averages[i], features[i].clone()))
        .collect();

    // Sort by cluster size descending and show top 10
    cluster_info.sort_by(|a, b| b.1.cmp(&a.1));
    println!("Top 5 largest clusters with average scores and features:\n");
    for (i, (id, size, avg, feats)) in cluster_info.iter().take(5).enumerate() {
        println!("Rank {:<2}: Cluster {:<3} ({} students)\n  Avg score: {:.2}", i + 1, id, size, avg);
        for (label, val) in feats {
            if !label.contains("placeholder") {
                println!("    {:<30} = {:.2}", label, val);
            }
        }
        println!();
    }

    Ok(())
}


// Intermediate commit: added comment for milestone tracking

