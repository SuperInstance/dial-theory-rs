# Integration Guide: dial-theory-rs

## What This Crate Provides

Cultural dial positions for agent personality — where agents fall on theoretical spectrums. Models cultural traditions as points on 2D dials with distance metrics, clustering, and evolution over time.

- **`position::DialPosition`** — 2D coordinate on a cultural dial `[-1, 1] × [-1, 1]`. Methods: `new()`, `center()`, `normalized()`, `distance_to()`, `midpoint()`, `weighted_avg()`, `angle()`, `magnitude()`, `move_toward()`, `is_valid()`.
- **`tradition::Tradition`** — Cultural tradition with `name`, `position`, `strength` (0–1), and `description`. Methods: `new()`, `with_strength()`, `describe()`, `distance_to()`, `is_similar()`, `blend()`.
- **`tradition::TraditionSet`** — Collection of traditions forming a cultural landscape. Methods: `new()`, `add()`, `remove()`, `find_by_name()`, `closest_to()`, `diversity()`.
- **`distance::DistanceMetric`** — Enum: `Euclidean`, `Manhattan`, `Chebyshev`, `Angular`, `Cosine`.
- **`distance::distance()`** — Compute distance between two positions using any metric.
- **`distance::tradition_distance()`** — Distance between two traditions.
- **`distance::distance_matrix()`** — Full pairwise distance matrix for a set of positions.
- **`distance::closest_pair()`** — Find the two closest positions and their distance.
- **`clustering::Cluster`** — A cluster with `centroid` and `members` indices. Methods: `new()`, `size()`, `recompute_centroid()`.
- **`clustering::ClusteringResult`** — Result with `clusters`, `iterations`, `converged`.
- **`clustering::kmeans()`** — K-means clustering on traditions using any distance metric.
- **`evolution::evolve_tradition()`** — Evolve a tradition's position over time with drift and attraction.
- **`evolution::evolve_population()`** — Evolve an entire tradition set through generations.

## How to Add This Crate

```bash
cargo add dial-theory
```

```rust
use dial_theory::{
    DialPosition, Tradition, TraditionSet,
    DistanceMetric, distance_matrix, kmeans,
};
```

## Cross-Repo Connections

### With `conservation-law-rs`: Cultural Energy Conservation

Treat cultural distance from center as potential energy, conserved during tradition blending:

```rust
use dial_theory::{DialPosition, Tradition};
use conservation_law::lagrangian::AgentState;

fn blend_conserving_energy(a: &Tradition, b: &Tradition) -> Tradition {
    let energy_before = a.position.magnitude().powi(2) + b.position.magnitude().powi(2);
    let blended_pos = a.blend(b);
    let energy_after = blended_pos.magnitude().powi(2) * 2.0;
    
    // Scale to conserve cultural energy
    let scale = (energy_before / energy_after).sqrt();
    let conserved_pos = DialPosition::new(
        blended_pos.x * scale,
        blended_pos.y * scale,
    ).normalized();
    
    Tradition::new("blended", conserved_pos)
        .with_strength((a.strength + b.strength) / 2.0)
}
```

### With `si-cli`: Interactive Tradition Browser

Browse and compare agent cultural positions via the CLI:

```rust
use dial_theory::{TraditionSet, Tradition, DialPosition, DistanceMetric};

fn cli_tradition_distance(set: &TraditionSet, name_a: &str, name_b: &str) {
    let a = set.find_by_name(name_a).unwrap();
    let b = set.find_by_name(name_b).unwrap();
    
    for metric in [DistanceMetric::Euclidean, DistanceMetric::Cosine, DistanceMetric::Angular] {
        let d = dial_theory::tradition_distance(a, b, &metric);
        println!("{:?}: {:.4}", metric, d);
    }
}

fn cli_cluster_traditions(set: &TraditionSet, k: usize) {
    let result = kmeans(&set.traditions, k, 100, &DistanceMetric::Euclidean);
    println!("Converged: {} in {} iterations", result.converged, result.iterations);
    for (i, cluster) in result.clusters.iter().enumerate() {
        println!("Cluster {}: {} members at ({:.2}, {:.2})",
            i, cluster.size(), cluster.centroid.x, cluster.centroid.y);
    }
}
```

### With `si-fleet-api`: REST Cultural Analysis

Expose tradition clustering and distance analysis via REST:

```rust
use dial_theory::{TraditionSet, Tradition, DialPosition, kmeans, DistanceMetric};
use si_fleet_api::{HttpRequest, HttpResponse};

fn get_tradition_clusters(req: HttpRequest) -> HttpResponse {
    let body: serde_json::Value = req.json().unwrap();
    let traditions: Vec<Tradition> = serde_json::from_value(body["traditions"].clone()).unwrap();
    let k = body["k"].as_u64().unwrap() as usize;
    
    let result = kmeans(&traditions, k, 100, &DistanceMetric::Euclidean);
    
    HttpResponse::json(json!({
        "converged": result.converged,
        "iterations": result.iterations,
        "clusters": result.clusters.iter().map(|c| json!({
            "centroid": { "x": c.centroid.x, "y": c.centroid.y },
            "size": c.size(),
            "members": c.members,
        })).collect::<Vec<_>>(),
    }))
}

fn post_tradition_blend(req: HttpRequest) -> HttpResponse {
    let body: serde_json::Value = req.json().unwrap();
    let a: Tradition = serde_json::from_value(body["a"].clone()).unwrap();
    let b: Tradition = serde_json::from_value(body["b"].clone()).unwrap();
    let blended = a.blend(&b);
    
    HttpResponse::json(json!({
        "x": blended.x,
        "y": blended.y,
        "distance_from_a": a.position.distance_to(&blended),
        "distance_from_b": b.position.distance_to(&blended),
    }))
}
```

### With Supabase: Persistent Cultural Landscapes

Store agent cultural positions and track evolution over time in Supabase:

```rust
use dial_theory::{Tradition, DialPosition};
use supabase_rs::SupabaseClient;

async fn persist_tradition(
    client: &SupabaseClient,
    agent_id: &str,
    tradition: &Tradition,
) {
    client.from("agent_traditions")
        .insert(json!({
            "agent_id": agent_id,
            "tradition_name": tradition.name,
            "position_x": tradition.position.x,
            "position_y": tradition.position.y,
            "strength": tradition.strength,
            "description": tradition.description,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }))
        .execute()
        .await
        .unwrap();
}

async fn get_tradition_timeline(
    client: &SupabaseClient,
    agent_id: &str,
    tradition_name: &str,
) -> Vec<(DialPosition, f64, String)> {
    let rows = client.from("agent_traditions")
        .select("*")
        .eq("agent_id", agent_id)
        .eq("tradition_name", tradition_name)
        .order("timestamp.asc")
        .execute()
        .await
        .unwrap();
    
    rows.into_iter()
        .map(|r| (
            DialPosition::new(
                r.get("position_x").unwrap().parse().unwrap(),
                r.get("position_y").unwrap().parse().unwrap(),
            ),
            r.get("strength").unwrap().parse().unwrap(),
            r.get("timestamp").unwrap().to_string(),
        ))
        .collect()
}

async fn get_fleet_cultural_centroid(client: &SupabaseClient) -> DialPosition {
    let rows = client.from("agent_traditions")
        .select("position_x,position_y")
        .execute()
        .await
        .unwrap();
    
    let n = rows.len() as f64;
    let x: f64 = rows.iter().map(|r| r.get("position_x").unwrap().parse::<f64>().unwrap()).sum();
    let y: f64 = rows.iter().map(|r| r.get("position_y").unwrap().parse::<f64>().unwrap()).sum();
    
    DialPosition::new(x / n, y / n)
}
```

## Design Patterns

### Pattern: Cultural Drift Tracking

Monitor how an agent's dial position drifts over time and alert on rapid change:

```rust
use dial_theory::{DialPosition, Tradition};

fn cultural_drift_alert(history: &[DialPosition], threshold: f64) -> bool {
    if history.len() < 2 {
        return false;
    }
    let drift = history.last().unwrap().distance_to(&history[0]);
    drift > threshold
}
```

### Pattern: Consensus-Based Blending

Blend multiple traditions by weighted democratic consensus:

```rust
use dial_theory::{Tradition, DialPosition};

fn consensus_blend(traditions: &[Tradition]) -> DialPosition {
    let total_strength: f64 = traditions.iter().map(|t| t.strength).sum();
    let x: f64 = traditions.iter().map(|t| t.position.x * t.strength).sum::<f64>() / total_strength;
    let y: f64 = traditions.iter().map(|t| t.position.y * t.strength).sum::<f64>() / total_strength;
    DialPosition::new(x, y).normalized()
}
```

### Pattern: Affinity-Based Clustering

Use spectral clustering on tradition distance matrices to find natural cultural groupings:

```rust
use dial_theory::{TraditionSet, DistanceMetric, distance_matrix};

fn tradition_affinity_clusters(set: &TraditionSet, k: usize) -> Vec<Vec<usize>> {
    let positions: Vec<_> = set.traditions.iter().map(|t| t.position.clone()).collect();
    let dist = distance_matrix(&positions, &DistanceMetric::Cosine);
    
    // Convert distance to affinity
    let affinity: Vec<Vec<f64>> = dist.iter()
        .map(|row| row.iter().map(|&d| (-d * d).exp()).collect())
        .collect();
    
    // Run spectral clustering (pseudo-code)
    // spectral_clustering(&affinity, k, ...)
    vec![]
}
```
