//! Tutorial: dial-theory-rs — Cultural Dial Positions for Agent Personality
//!
//! Shows how to place traditions on a 2D cultural dial, measure distances,
//! cluster related traditions, and simulate their evolution over time.

use dial_theory_rs::{
    position::DialPosition,
    tradition::{Tradition, TraditionSet},
    distance::DistanceMetric,
    clustering,
    evolution::{EvolutionTimeline, simulate_drift},
};

fn main() {
    println!("=== Dial Theory Tutorial ===\n");

    // Part 1: Create dial positions
    println!("Part 1: Dial positions");
    let rational = DialPosition::new(1.0, 0.0);
    let empirical = DialPosition::new(0.0, 1.0);
    let romantic = DialPosition::new(-1.0, 0.0);
    println!("  Rational: ({}, {})", rational.x, rational.y);
    println!("  Empirical: ({}, {})", empirical.x, empirical.y);
    println!("  Romantic: ({}, {})", romantic.x, romantic.y);
    println!();

    // Part 2: Create traditions with positions
    println!("Part 2: Traditions on the dial");
    let formal = Tradition::new("Formal Verification", DialPosition::new(0.9, 0.3))
        .describe("Mathematical proof of correctness");
    let ml = Tradition::new("Machine Learning", DialPosition::new(-0.2, 0.8))
        .describe("Data-driven pattern recognition");
    let agile = Tradition::new("Agile Methods", DialPosition::new(-0.7, -0.4))
        .describe("Iterative people-first development");
    let systems = Tradition::new("Systems Engineering", DialPosition::new(0.5, -0.6))
        .describe("Holistic system design");
    
    println!("  {} at ({:.1}, {:.1})", formal.name, formal.position.x, formal.position.y);
    println!("  {} at ({:.1}, {:.1})", ml.name, ml.position.x, ml.position.y);
    println!();

    // Part 3: Distance metrics
    println!("Part 3: Distances between traditions");
    let eucl = formal.distance_to(&ml);
    println!("  Formal → ML (Euclidean): {:.3}", eucl);
    
    let similar = formal.is_similar(&systems, 1.5);
    println!("  Formal ≈ Systems (within 1.5)? {}", similar);
    
    let blended = formal.blend(&ml);
    println!("  Formal ⊕ ML blend: ({:.2}, {:.2})", blended.x, blended.y);
    println!();

    // Part 4: Tradition sets and clustering
    println!("Part 4: Clustering traditions");
    let mut set = TraditionSet::new();
    set.traditions.push(formal);
    set.traditions.push(ml);
    set.traditions.push(agile);
    set.traditions.push(systems);
    set.traditions.push(Tradition::new("Type Theory", DialPosition::new(0.85, 0.25)));
    set.traditions.push(Tradition::new("Neural Nets", DialPosition::new(-0.15, 0.85)));
    
    let result = clustering::kmeans(&set.traditions, 2, 10, &DistanceMetric::Euclidean);
    println!("  {} clusters found", result.clusters.len());
    for (i, cluster) in result.clusters.iter().enumerate() {
        println!("    Cluster {}: {} members", i, cluster.size());
    }
    
    // Elbow analysis for optimal k
    let elbow = clustering::elbow_analysis(&set.traditions, 5, &DistanceMetric::Euclidean);
    println!("  Inertias by k: {:?}", elbow);
    println!();

    // Part 5: Evolution timeline
    println!("Part 5: Tradition evolution");
    let mut timeline = EvolutionTimeline::new("DevOps", DialPosition::new(-0.5, -0.3));
    timeline.record(1, DialPosition::new(-0.4, -0.1));
    timeline.record(2, DialPosition::new(-0.2, 0.1));
    timeline.record(3, DialPosition::new(0.0, 0.3));
    println!("  Total drift: {:.3}", timeline.total_distance());
    println!("  Avg speed: {:.3}", timeline.average_speed());
    println!("  Displacement: {:.3}", timeline.displacement());
    println!("  Current: ({:.2}, {:.2})", timeline.current().x, timeline.current().y);
    println!();

    // Part 6: Simulated drift
    println!("Part 6: Simulated tradition drift");
    let targets: Vec<DialPosition> = (0..set.traditions.len())
        .map(|_| DialPosition::new(0.0, 0.0))
        .collect();
    let drifted = simulate_drift(&set.traditions, &targets, 0.1, 50);
    println!("  {} positions drifted over 50 steps", drifted.len());
}
