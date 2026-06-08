# dial-theory-rs

Cultural dial positions for agent personality — where agents fall on theoretical spectrums.

Models cultural traditions as positions in a 2D continuous space. Traditions cluster, drift, merge, split, and compete — the same dynamics that govern real cultural ecosystems, now available as composable Rust primitives.

Part of the **sunset-ecosystem**: tradition positions feed into `conservation-law` (which enforces resource conservation as traditions evolve), and fleet-level coordination uses `si-fleet-api` to propagate dial state across agents.

## The Math

### Dial Positions as Points on a Manifold

Each tradition occupies a position $(x, y) \in [-1, 1]^2$ on a cultural dial. The distance between two traditions is:

$$d(p_1, p_2) = \sqrt{(x_1 - x_2)^2 + (y_1 - y_2)^2}$$

This is Euclidean distance, but we also support Manhattan ($L_1$), Chebyshev ($L_\infty$), angular, and cosine metrics — each reveals a different aspect of cultural topology.

### Lotka-Volterra Competition

When two traditions occupy similar dial positions, they compete. The Lotka-Volterra competition model describes how tradition strengths evolve:

$$\frac{ds_i}{dt} = r_i s_i \left(1 - \frac{s_i + \alpha_{ij} s_j}{K_i}\right)$$

where $s_i$ is the strength of tradition $i$, $r_i$ its growth rate, $K_i$ its carrying capacity, and $\alpha_{ij}$ the competition coefficient encoding how much tradition $j$ suppresses $i$.

### K-Means Clustering

Traditions cluster by proximity using k-means. The algorithm minimizes inertia:

$$J = \sum_{k=1}^{K} \sum_{i \in C_k} d(p_i, \mu_k)^2$$

where $\mu_k$ is the centroid of cluster $C_k$.

### Tradition Evolution as a Dynamical System

Traditions evolve via four operations:
- **Drift**: move toward a target at rate $\alpha$: $p_{t+1} = (1 - \alpha) p_t + \alpha \cdot p_{\text{target}}$
- **Merge**: weighted average of two traditions
- **Split**: spawn a new tradition at an offset
- **Pressure**: external force pushing toward a source point

## Installation

```toml
[dependencies]
dial-theory-rs = { git = "https://github.com/SuperInstance/dial-theory-rs" }
```

## Usage

### Creating Traditions and Measuring Distance

```rust
use dial_theory_rs::position::DialPosition;
use dial_theory_rs::tradition::{Tradition, TraditionSet};
use dial_theory_rs::distance::{distance, DistanceMetric};

// Define traditions at cultural positions
let stoicism = Tradition::new("stoicism", DialPosition::new(0.8, -0.3))
    .describe("Virtue ethics, emotional resilience");
let epicureanism = Tradition::new("epicureanism", DialPosition::new(0.2, 0.5))
    .describe("Pleasure as absence of suffering");

// Euclidean distance between traditions
let d = stoicism.distance_to(&epicureanism);
println!("Distance: {:.3}", d);

// Cosine similarity — are they pointing the same direction?
let cos_dist = distance(
    &stoicism.position,
    &epicureanism.position,
    &DistanceMetric::Cosine,
);
println!("Cosine distance: {:.3}", cos_dist);

// Weighted blend — what if these traditions merge?
let blended = stoicism.blend(&epicureanism);
println!("Blended position: {}", blended);
```

### Tradition Sets and Cultural Landscapes

```rust
use dial_theory_rs::tradition::{Tradition, TraditionSet};
use dial_theory_rs::position::DialPosition;

let mut landscape = TraditionSet::new();
landscape.add(Tradition::new("stoicism", DialPosition::new(0.8, -0.3)));
landscape.add(Tradition::new("epicureanism", DialPosition::new(0.2, 0.5)));
landscape.add(Tradition::new("cynicism", DialPosition::new(-0.9, -0.8)));
landscape.add(Tradition::new("platonism", DialPosition::new(0.6, 0.7)));

// Find the cultural center of gravity
let centroid = landscape.centroid();
println!("Cultural centroid: {}", centroid);

// Find nearest tradition to a position
let query = DialPosition::new(0.5, 0.5);
let nearest = landscape.nearest(&query).unwrap();
println!("Nearest to query: {} at {}", nearest.name, nearest.position);

// Find all traditions within radius 0.5 of a point
let nearby = landscape.within_radius(&DialPosition::new(0.0, 0.0), 0.5);
println!("Traditions near origin: {} found", nearby.len());
```

### Clustering Traditions

```rust
use dial_theory_rs::tradition::Tradition;
use dial_theory_rs::position::DialPosition;
use dial_theory_rs::clustering::{kmeans, elbow_analysis, cluster_representatives};
use dial_theory_rs::distance::DistanceMetric;

let traditions = vec![
    Tradition::new("western_individualism", DialPosition::new(0.9, 0.2)),
    Tradition::new("eastern_collectivism", DialPosition::new(-0.8, -0.3)),
    Tradition::new("stoicism", DialPosition::new(0.7, -0.2)),
    Tradition::new("confucianism", DialPosition::new(-0.7, -0.4)),
    Tradition::new("existentialism", DialPosition::new(0.6, 0.8)),
    Tradition::new("buddhism", DialPosition::new(-0.3, 0.6)),
];

// Cluster into 3 groups
let result = kmeans(&traditions, 3, 100, &DistanceMetric::Euclidean);
println!("Converged: {}, iterations: {}", result.converged, result.iterations);

for (i, cluster) in result.clusters.iter().enumerate() {
    println!("Cluster {}: centroid={}, {} members", i, cluster.centroid, cluster.size());
}

// Elbow analysis — how many clusters are natural?
let elbow = elbow_analysis(&traditions, 5, &DistanceMetric::Euclidean);
println!("\nElbow analysis:");
for (k, inertia) in &elbow {
    println!("  k={}: inertia={:.4}", k, inertia);
}

// Find the most representative tradition in each cluster
let reps = cluster_representatives(&traditions, &result.clusters, &DistanceMetric::Euclidean);
for (i, rep) in reps.iter().enumerate() {
    if let Some(idx) = rep {
        println!("Cluster {} representative: {}", i, traditions[*idx].name);
    }
}
```

### Distance Metrics

```rust
use dial_theory_rs::position::DialPosition;
use dial_theory_rs::distance::{distance, distance_matrix, closest_pair, DistanceMetric};

let p1 = DialPosition::new(0.5, 0.5);
let p2 = DialPosition::new(-0.3, 0.8);

// Five distance metrics available
let euclidean = distance(&p1, &p2, &DistanceMetric::Euclidean);
let manhattan  = distance(&p1, &p2, &DistanceMetric::Manhattan);
let chebyshev  = distance(&p1, &p2, &DistanceMetric::Chebyshev);
let angular    = distance(&p1, &p2, &DistanceMetric::Angular);
let cosine     = distance(&p1, &p2, &DistanceMetric::Cosine);

println!("Euclidean: {:.3}", euclidean);
println!("Manhattan:  {:.3}", manhattan);
println!("Chebyshev:  {:.3}", chebyshev);
println!("Angular:    {:.3}", angular);
println!("Cosine:     {:.3}", cosine);

// Build a full pairwise distance matrix
let positions = vec![
    DialPosition::new(0.0, 0.0),
    DialPosition::new(1.0, 0.0),
    DialPosition::new(0.0, 1.0),
];
let matrix = distance_matrix(&positions, &DistanceMetric::Euclidean);
println!("\nDistance matrix:");
for row in &matrix {
    println!("  {:?}", row);
}

// Find the closest pair
let points = vec![
    DialPosition::new(0.0, 0.0),
    DialPosition::new(10.0, 10.0),
    DialPosition::new(0.1, 0.1),
];
let (i, j, d) = closest_pair(&points, &DistanceMetric::Euclidean);
println!("\nClosest pair: {} and {} at distance {:.3}", i, j, d);
```

### Tradition Evolution Over Time

```rust
use dial_theory_rs::tradition::Tradition;
use dial_theory_rs::position::DialPosition;
use dial_theory_rs::evolution::{
    EvolutionEvent, EvolutionTimeline, apply_event,
    simulate_drift, simulate_convergence,
};

// Single evolution event: drift toward a target
let tradition = Tradition::new("reform", DialPosition::new(0.0, 0.0));
let drift = EvolutionEvent::Drift {
    target: DialPosition::new(1.0, 0.0),
    rate: 0.3,
};
let new_pos = apply_event(&tradition, &drift);
println!("After drift: {}", new_pos);

// External pressure event
let pressure = EvolutionEvent::Pressure {
    source: DialPosition::new(0.5, 0.5),
    strength: 0.1,
};
let pressured = apply_event(&tradition, &pressure);
println!("After pressure: {}", pressured);

// Split: a tradition forks
let split = EvolutionEvent::Split {
    offset: DialPosition::new(0.3, -0.3),
};
let forked = apply_event(&tradition, &split);
println!("After split: {}", forked);

// Simulate drift of multiple traditions toward targets
let traditions = vec![
    Tradition::new("conservative", DialPosition::new(-0.8, 0.0)),
    Tradition::new("progressive", DialPosition::new(0.8, 0.0)),
];
let targets = vec![
    DialPosition::center(), // both drift toward center
    DialPosition::center(),
];
let timelines = simulate_drift(&traditions, &targets, 0.1, 20);
for tl in &timelines {
    println!("{}: traveled {:.3}, now at {}",
        tl.name, tl.total_distance(), tl.current());
}

// Convergent evolution — all traditions drift to centroid
let many = vec![
    Tradition::new("a", DialPosition::new(-0.8, -0.8)),
    Tradition::new("b", DialPosition::new(0.8, -0.8)),
    Tradition::new("c", DialPosition::new(0.0, 0.8)),
];
let converging = simulate_convergence(&many, 0.15, 30);
for tl in &converging {
    println!("{}: displacement={:.3}, avg_speed={:.4}",
        tl.name, tl.displacement(), tl.average_speed());
}
```

### Building an Evolution Timeline

```rust
use dial_theory_rs::position::DialPosition;
use dial_theory_rs::evolution::EvolutionTimeline;

// Track a tradition's position over time
let mut timeline = EvolutionTimeline::new("greek_philosophy", DialPosition::new(0.5, 0.5));

// Record positions at different timesteps
timeline.record(1, DialPosition::new(0.52, 0.48));
timeline.record(2, DialPosition::new(0.55, 0.45));
timeline.record(3, DialPosition::new(0.58, 0.42));
timeline.record(4, DialPosition::new(0.60, 0.40));

println!("Total distance traveled: {:.4}", timeline.total_distance());
println!("Net displacement: {:.4}", timeline.displacement());
println!("Average speed: {:.4}", timeline.average_speed());
println!("Current position: {}", timeline.current());
```

### Weighted Traditions

```rust
use dial_theory_rs::tradition::Tradition;
use dial_theory_rs::position::DialPosition;

// Traditions with different adherence strengths
let strong = Tradition::with_strength("core_value", DialPosition::new(0.9, 0.9), 0.95);
let weak   = Tradition::with_strength("influence", DialPosition::new(-0.5, 0.3), 0.2);

// Blend respects strength weights
let blend = strong.blend(&weak);
// Strong tradition dominates the blend
println!("Blended position: {} (pulled toward strong)", blend);

// Check similarity
println!("Similar? {}", strong.is_similar(&weak, 0.5));
```

## API Reference

### `DialPosition`

| Method | Description |
|--------|-------------|
| `new(x, y)` | Create position at $(x, y)$ |
| `center()` | The neutral origin $(0, 0)$ |
| `normalized()` | Clamp to $[-1, 1]^2$ |
| `distance_to(other)` | Euclidean distance |
| `midpoint(other)` | Midpoint between two positions |
| `weighted_avg(other, w)` | Weighted average |
| `move_toward(target, rate)` | Interpolate toward target |
| `angle()` | Angle from origin (radians) |
| `magnitude()` | Distance from origin |
| `is_valid()` | Check bounds |

### `Tradition`

| Method | Description |
|--------|-------------|
| `new(name, position)` | Create a named tradition |
| `with_strength(name, pos, s)` | Tradition with adherence strength $s \in [0, 1]$ |
| `distance_to(other)` | Distance to another tradition |
| `is_similar(other, threshold)` | Check proximity |
| `blend(other)` | Weighted position average |

### `TraditionSet`

| Method | Description |
|--------|-------------|
| `add(tradition)` | Add a tradition |
| `nearest(position)` | Find closest tradition |
| `centroid()` | Mean position of all traditions |
| `within_radius(pos, r)` | Find nearby traditions |

### Distance Metrics

| Variant | Formula |
|---------|---------|
| `Euclidean` | $\sqrt{\sum (a_i - b_i)^2}$ |
| `Manhattan` | $\sum \|a_i - b_i\|$ |
| `Chebyshev` | $\max_i \|a_i - b_i\|$ |
| `Angular` | Shortest angular distance |
| `Cosine` | $1 - \frac{a \cdot b}{\|a\| \|b\|}$ |

### Clustering

| Function | Description |
|----------|-------------|
| `kmeans(traditions, k, max_iter, metric)` | K-means clustering |
| `elbow_analysis(traditions, max_k, metric)` | Inertia for $k = 1 \ldots k_{\max}$ |
| `cluster_representatives(traditions, clusters, metric)` | Most central member per cluster |

### Evolution

| Function | Description |
|----------|-------------|
| `apply_event(tradition, event)` | Apply one evolution step |
| `simulate_drift(traditions, targets, rate, steps)` | Drift toward targets |
| `simulate_convergence(traditions, rate, steps)` | All drift to centroid |

## Why This Matters for Agent Systems

Agents aren't stateless — they carry cultural context. When a fleet of agents coordinates via `si-fleet-api`, each agent's dial position determines:

1. **Coalition formation**: agents with similar dial positions cluster naturally (k-means on traditions)
2. **Resource allocation**: `conservation-law` enforces that as traditions shift, total cultural "energy" is conserved
3. **Drift detection**: evolution timelines track whether an agent's cultural position has moved too far from its assigned cluster
4. **Merge/split decisions**: when two traditions are close enough, they merge; when one spans too wide a dial, it splits

The dial is a low-dimensional embedding of high-dimensional cultural state — it makes the intractable (full belief systems) tractable (2D position) while preserving the dynamics that matter.

## Integration

### With `conservation-law`

```rust
// conservation-law tracks total dial "energy"
// When traditions evolve, total energy must be conserved
// dial-theory-rs provides the positions; conservation-law enforces the constraints
```

### With `si-fleet-api`

```rust
// Fleet agents report their dial positions via si-fleet-api
// Fleet coordinator runs clustering to group agents by cultural alignment
// Transport plans (from wasserstein-agents-rs) optimize agent redistribution
```

### Lotka-Volterra Tradition Competition

When two traditions occupy nearby positions on the dial, their strengths compete:

```rust
use dial_theory_rs::tradition::Tradition;
use dial_theory_rs::position::DialPosition;

// Two competing traditions with different strengths
let dominant = Tradition::with_strength("established", DialPosition::new(0.5, 0.5), 0.9);
let challenger = Tradition::with_strength("emerging", DialPosition::new(0.55, 0.52), 0.3);

// The blend is dominated by the stronger tradition
let blend = dominant.blend(&challenger);
println!("Competition blend: {}", blend);

// Distance between them is small → direct competition
let d = dominant.distance_to(&challenger);
println!("Competition distance: {:.3}", d);

// A third tradition far away faces no competition
let distant = Tradition::new("isolated", DialPosition::new(-0.8, -0.8));
println!("Distance to distant: {:.3}", dominant.distance_to(&distant));
```

### DialPosition Geometry

```rust
use dial_theory_rs::position::DialPosition;

let p = DialPosition::new(3.0, 4.0);

// Magnitude: distance from origin
println!("Magnitude: {:.1}", p.magnitude()); // 5.0

// Angle: direction from origin
println!("Angle: {:.3} rad", p.angle()); // atan2(4, 3)

// Normalized to unit square
let n = p.normalized();
println!("Normalized: ({:.1}, {:.1})", n.x, n.y); // (1.0, 1.0)

// Check validity
assert!(!p.is_valid());  // outside [-1, 1]²
assert!(n.is_valid());   // inside [-1, 1]²

// Midpoint between two positions
let a = DialPosition::new(-1.0, 0.0);
let b = DialPosition::new(1.0, 0.0);
let mid = a.midpoint(&b);
println!("Midpoint: {}", mid); // (0.000, 0.000)
```

## License

MIT
