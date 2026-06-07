# INTEGRATION.md — dial-theory-rs × wasserstein-agents-rs × sheaf-coherence-rs

**Measuring disagreement**: dial-theory positions agents on cultural spectrums, wasserstein-agents measures the distributional distance between agent populations, and sheaf-coherence checks if local agreement assembles into global consensus.

## Synergy Map

```
dial-theory-rs            wasserstein-agents-rs       sheaf-coherence-rs
┌──────────────────┐     ┌─────────────────────┐     ┌─────────────────────┐
│ DialPosition     │     │ AgentDistribution    │     │ Sheaf               │
│ Tradition        │────►│ SinkhornSolver       │────►│ RestrictionMap      │
│ DistanceMetric   │     │ OptimalTransport     │     │ Assignment          │
│  Euclidean       │     │ JKOScheme            │     │ coherence_energy()  │
│  Angular         │     │ barycenter_sinkhorn  │     │ normalized_energy() │
│  Cosine          │     │ sliced_wasserstein_* │     │ synchronization     │
│ Clustering       │     │ dist_w2              │     │ sheaf_laplacian     │
│ Evolution        │     │ AgentDistribution    │     │ Laplacian           │
└──────────────────┘     └─────────────────────┘     └─────────────────────┘
         │                        │                          │
         └────────────────────────┼──────────────────────────┘
                                  ▼
                    Measure how agents disagree:
                    positions → transport → coherence
```

## Key Insight

Three levels of disagreement measurement:
1. **dial-theory**: point-level — where does *this* agent sit on *this* spectrum?
2. **wasserstein-agents**: distribution-level — how far apart are two *populations* of agents?
3. **sheaf-coherence**: structural-level — does local agreement add up to global consensus?

Use all three to get a complete picture of multi-agent disagreement.

## Example 1: Multi-Scale Agent Disagreement

```rust
use dial_theory_rs::position::DialPosition;
use dial_theory_rs::distance::{distance, DistanceMetric};
use wasserstein_agents_rs::transport::{SinkhornSolver, OptimalTransport};
use wasserstein_agents_rs::sliced::sliced_wasserstein_1;
use sheaf_coherence_rs::sheaf::{Cell, RestrictionMap, Assignment, Sheaf};
use sheaf_coherence_rs::coherence::{coherence_energy, normalized_coherence_energy};

/// Measure disagreement at all three scales for a group of agents.
fn measure_disagreement(
    positions: &[DialPosition],
    group_a_weights: &[f64],
    group_b_weights: &[f64],
) {
    // === Level 1: Pairwise dial distances ===
    println!("=== Level 1: Pairwise Disagreement ===");
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let euclidean = distance(&positions[i], &positions[j], &DistanceMetric::Euclidean);
            let angular = distance(&positions[i], &positions[j], &DistanceMetric::Angular);
            let cosine = distance(&positions[i], &positions[j], &DistanceMetric::Cosine);
            println!("  Agent {} vs {}: euclidean={:.3} angular={:.3} cosine={:.3}",
                i, j, euclidean, angular, cosine);
        }
    }

    // === Level 2: Distributional distance (Wasserstein) ===
    println!("\n=== Level 2: Population Distance ===");
    let x: Vec<f64> = positions.iter().map(|p| p.x).collect();
    let y: Vec<f64> = positions.iter().map(|p| p.y).collect();

    let sw1 = sliced_wasserstein_1(&x, &y, 100);
    println!("  Sliced Wasserstein-1 (x-coords): {:.4}", sw1);

    // Build cost matrix for Sinkhorn
    let n = group_a_weights.len();
    let m = group_b_weights.len();
    let cost: Vec<Vec<f64>> = (0..n)
        .map(|i| (0..m)
            .map(|j| {
                let dx = if i < positions.len() && j < positions.len() {
                    positions[i].x - positions[j].x
                } else { 0.0 };
                dx.abs()
            })
            .collect())
        .collect();

    let solver = SinkhornSolver::new(0.1);
    let transport_plan = solver.solve(&cost, group_a_weights, group_b_weights);
    let total_mass: f64 = transport_plan.iter().flat_map(|r| r.iter()).sum();
    println!("  Transport plan total mass: {:.4}", total_mass);

    // === Level 3: Structural coherence ===
    println!("\n=== Level 3: Structural Coherence ===");
    let mut sheaf = Sheaf::new();
    for (i, pos) in positions.iter().enumerate() {
        sheaf.add_cell(Cell::new(i, 0));
        sheaf.assign(i, Assignment::new(vec![pos.x, pos.y]));
    }

    // Restriction maps: identity (simplified agreement)
    for i in 0..positions.len().saturating_sub(1) {
        sheaf.add_restriction_map(RestrictionMap::new(
            i, i + 1,
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
        ));
    }

    let energy = coherence_energy(&sheaf);
    let normalized = normalized_coherence_energy(&sheaf);
    println!("  Coherence energy: {:.4} (lower = more coherent)", energy);
    println!("  Normalized:       {:.4}", normalized);
    println!("  Global consensus: {:.0}%", (1.0 - normalized.min(1.0)) * 100.0);
}

fn main() {
    let positions = vec![
        DialPosition::new(0.8, 0.2),   // Agent 0: tech-leaning
        DialPosition::new(-0.5, 0.7),   // Agent 1: humanities-leaning
        DialPosition::new(0.3, -0.4),   // Agent 2: moderate, skeptical
        DialPosition::new(0.9, 0.9),    // Agent 3: strongly positive
        DialPosition::new(-0.8, -0.6),  // Agent 4: contrarian
    ];

    let group_a = vec![0.3, 0.2, 0.1, 0.25, 0.15]; // normalized weights
    let group_b = vec![0.1, 0.3, 0.2, 0.15, 0.25]; // different emphasis

    measure_disagreement(&positions, &group_a, &group_b);
}
```

## Example 2: Wasserstein Barycenter of Agent Traditions

Find the "center of mass" of multiple agent traditions using optimal transport:

```rust
use dial_theory_rs::position::DialPosition;
use dial_theory_rs::tradition::Tradition;
use dial_theory_rs::distance::{distance, DistanceMetric};
use wasserstein_agents_rs::barycenter::{barycenter_1d_quantile, dist_w2};
use wasserstein_agents_rs::sliced::sliced_wasserstein_2;

/// Compute the Wasserstein barycenter of agent positions.
fn tradition_barycenter(positions: &[DialPosition]) -> DialPosition {
    let xs: Vec<f64> = positions.iter().map(|p| p.x).collect();
    let ys: Vec<f64> = positions.iter().map(|p| p.y).collect();

    // Simple barycenter: mean position
    let mean_x: f64 = xs.iter().sum::<f64>() / xs.len() as f64;
    let mean_y: f64 = ys.iter().sum::<f64>() / ys.len() as f64;

    println!("Barycenter position: ({:.3}, {:.3})", mean_x, mean_y);

    // Measure spread using sliced Wasserstein distance
    let sw = sliced_wasserstein_2(&xs, &ys, 50);
    println!("Cross-axis disagreement (SW2): {:.4}", sw);

    DialPosition::new(mean_x, mean_y)
}

/// Measure pairwise Wasserstein distances between traditions.
fn tradition_distance_matrix(traditions: &[DialPosition]) {
    println!("\n=== Tradition Distance Matrix ===");
    let n = traditions.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let d = distance(&traditions[i], &traditions[j], &DistanceMetric::Euclidean);
            println!("  T{} ↔ T{}: {:.4}", i, j, d);
        }
    }
}

fn main() {
    let traditions = vec![
        DialPosition::new(0.8, 0.2),   // Analytical
        DialPosition::new(-0.6, 0.7),   // Continental
        DialPosition::new(0.0, 0.0),    // Pragmatist
        DialPosition::new(0.5, -0.3),   // Empirical
    ];

    let center = tradition_barycenter(&traditions);
    tradition_distance_matrix(&traditions);

    println!("\nFleet center: ({:.2}, {:.2})", center.x, center.y);
    for (i, t) in traditions.iter().enumerate() {
        let d = distance(t, &center, &DistanceMetric::Euclidean);
        println!("  T{} distance from center: {:.3}", i, d);
    }
}
```

## Example 3: Sheaf Laplacian for Agent Consensus Detection

Use the sheaf Laplacian to detect which agents are blocking consensus:

```rust
use sheaf_coherence_rs::sheaf::{Cell, RestrictionMap, Assignment, Sheaf};
use sheaf_coherence_rs::coherence::{coherence_energy, normalized_coherence_energy, map_disagreement};
use sheaf_coherence_rs::laplacian::SheafLaplacian;
use dial_theory_rs::position::DialPosition;
use dial_theory_rs::distance::{distance, DistanceMetric};

/// Build a sheaf from agent positions and detect consensus blockers.
fn detect_consensus_blockers() {
    let agents = vec![
        DialPosition::new(0.8, 0.2),
        DialPosition::new(0.7, 0.3),
        DialPosition::new(0.6, 0.4),
        DialPosition::new(-0.9, -0.8),  // outlier / blocker
        DialPosition::new(0.5, 0.5),
    ];

    // Build sheaf: each agent is a cell, edges between close agents
    let mut sheaf = Sheaf::new();
    for (i, pos) in agents.iter().enumerate() {
        sheaf.add_cell(Cell::new(i, 0));
        sheaf.assign(i, Assignment::new(vec![pos.x, pos.y]));
    }

    // Add restriction maps for nearby agents (distance < 0.5)
    for i in 0..agents.len() {
        for j in (i + 1)..agents.len() {
            let d = distance(&agents[i], &agents[j], &DistanceMetric::Euclidean);
            if d < 0.8 {
                // Identity restriction map (agents agree on this overlap)
                sheaf.add_restriction_map(RestrictionMap::new(
                    i, j,
                    vec![vec![1.0, 0.0], vec![0.0, 1.0]],
                ));
            }
        }
    }

    // Compute coherence
    let energy = coherence_energy(&sheaf);
    let normalized = normalized_coherence_energy(&sheaf);
    println!("Fleet coherence: {:.4} (normalized: {:.4})", energy, normalized);

    // Identify blocker: agent 3 has high disagreement with others
    let blocker = &agents[3];
    for (i, agent) in agents.iter().enumerate() {
        if i == 3 { continue; }
        let d = distance(blocker, agent, &DistanceMetric::Euclidean);
        println!("  Blocker ↔ Agent {}: {:.3} {}", i, d,
            if d > 1.0 { "⚠ DISAGREE" } else { "✓" });
    }

    // Without the blocker
    let mut sheaf_clean = Sheaf::new();
    for (i, pos) in agents.iter().enumerate() {
        if i == 3 { continue; }
        let new_i = if i > 3 { i - 1 } else { i };
        sheaf_clean.add_cell(Cell::new(new_i, 0));
        sheaf_clean.assign(new_i, Assignment::new(vec![pos.x, pos.y]));
    }
    for i in 0..4 {
        for j in (i + 1)..4 {
            sheaf_clean.add_restriction_map(RestrictionMap::new(
                i, j,
                vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            ));
        }
    }
    let clean_energy = coherence_energy(&sheaf_clean);
    println!("\nWith blocker:    energy = {:.4}", energy);
    println!("Without blocker: energy = {:.4}", clean_energy);
    println!("Improvement:     {:.1}%", (energy - clean_energy) / energy * 100.0);
}

fn main() {
    detect_consensus_blockers();
}
```

## Data Flow

```
Agent positions (dial-theory)
         │
    ┌────┼────┐
    ▼    ▼    ▼
Euclidean Angular Cosine
distance distance distance
    │    │    │      ← Level 1: pairwise
    └────┼────┘
         ▼
AgentDistribution (wasserstein-agents)
    ├─ SinkhornSolver.solve() → transport plan
    ├─ sliced_wasserstein_1/2 → distribution distance
    └─ barycenter → population center
         │                       ← Level 2: distributional
         ▼
Cell + RestrictionMap (sheaf-coherence)
    ├─ coherence_energy() → global disagreement
    ├─ map_disagreement() → per-edge disagreement
    └─ sheaf_laplacian → structural holes
                            ← Level 3: structural
```

## When to Use This Combination

- **Multi-agent governance**: measure and predict disagreement in agent populations
- **Fleet coordination**: identify blocker agents that prevent consensus
- **Cultural analysis**: position agents on theoretical spectrums and measure population-level distance
- **Consensus detection**: use all three levels to determine if agreement is possible
