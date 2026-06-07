//! Cluster traditions by proximity.
//!
//! Implements k-means-like clustering for traditions on the dial.

use crate::distance::{distance, DistanceMetric};
use crate::position::DialPosition;
use crate::tradition::Tradition;

/// A cluster of traditions.
#[derive(Debug, Clone)]
pub struct Cluster {
    /// Centroid of the cluster.
    pub centroid: DialPosition,
    /// Indices of traditions in this cluster.
    pub members: Vec<usize>,
}

impl Cluster {
    /// Create an empty cluster at a centroid.
    pub fn new(centroid: DialPosition) -> Self {
        Cluster {
            centroid,
            members: Vec::new(),
        }
    }

    /// Number of members.
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Recompute the centroid from member positions.
    pub fn recompute_centroid(&mut self, traditions: &[Tradition]) {
        if self.members.is_empty() {
            return;
        }
        let n = self.members.len() as f64;
        let x: f64 = self.members.iter().map(|&i| traditions[i].position.x).sum::<f64>() / n;
        let y: f64 = self.members.iter().map(|&i| traditions[i].position.y).sum::<f64>() / n;
        self.centroid = DialPosition::new(x, y);
    }
}

/// Result of a clustering operation.
#[derive(Debug, Clone)]
pub struct ClusteringResult {
    /// The clusters found.
    pub clusters: Vec<Cluster>,
    /// Number of iterations performed.
    pub iterations: usize,
    /// Whether the algorithm converged.
    pub converged: bool,
}

/// Run k-means clustering on traditions.
///
/// # Arguments
/// * `traditions` - Slice of traditions to cluster
/// * `k` - Number of clusters
/// * `max_iter` - Maximum iterations
/// * `metric` - Distance metric to use
pub fn kmeans(
    traditions: &[Tradition],
    k: usize,
    max_iter: usize,
    metric: &DistanceMetric,
) -> ClusteringResult {
    assert!(k > 0, "k must be positive");
    assert!(traditions.len() >= k, "need at least k traditions");
    assert!(!traditions.is_empty(), "need at least one tradition");

    // Initialize centroids using evenly-spaced traditions
    let step = (traditions.len() as f64 / k as f64).max(1.0);
    let mut clusters: Vec<Cluster> = (0..k)
        .map(|i| {
            let idx = (i as f64 * step).min((traditions.len() - 1) as f64) as usize;
            Cluster::new(traditions[idx].position.clone())
        })
        .collect();

    let mut converged = false;
    let mut iterations = 0;

    for it in 0..max_iter {
        iterations = it + 1;

        // Clear member lists
        for c in &mut clusters {
            c.members.clear();
        }

        // Assign each tradition to nearest centroid
        for (i, t) in traditions.iter().enumerate() {
            let (best_idx, _) = clusters
                .iter()
                .enumerate()
                .map(|(ci, c)| (ci, distance(&t.position, &c.centroid, metric)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();
            clusters[best_idx].members.push(i);
        }

        // Recompute centroids and check convergence
        let mut total_shift = 0.0;
        for c in &mut clusters {
            let old_centroid = c.centroid.clone();
            c.recompute_centroid(traditions);
            total_shift += old_centroid.distance_to(&c.centroid);
        }

        if total_shift < 1e-6 {
            converged = true;
            break;
        }
    }

    ClusteringResult {
        clusters,
        iterations,
        converged,
    }
}

/// Find the optimal number of clusters using the elbow method.
/// Tests k from 1 to max_k and returns (k, inertia) pairs.
pub fn elbow_analysis(
    traditions: &[Tradition],
    max_k: usize,
    metric: &DistanceMetric,
) -> Vec<(usize, f64)> {
    let mut results = Vec::new();
    for k in 1..=max_k.min(traditions.len()) {
        let clustering = kmeans(traditions, k, 50, metric);
        let inertia = compute_inertia(traditions, &clustering.clusters, metric);
        results.push((k, inertia));
    }
    results
}

/// Compute total inertia (sum of squared distances to cluster centroids).
pub fn compute_inertia(
    traditions: &[Tradition],
    clusters: &[Cluster],
    metric: &DistanceMetric,
) -> f64 {
    let mut total = 0.0;
    for c in clusters {
        for &i in &c.members {
            let d = distance(&traditions[i].position, &c.centroid, metric);
            total += d * d;
        }
    }
    total
}

/// Find the most central tradition in each cluster.
pub fn cluster_representatives(
    traditions: &[Tradition],
    clusters: &[Cluster],
    metric: &DistanceMetric,
) -> Vec<Option<usize>> {
    clusters
        .iter()
        .map(|c| {
            c.members
                .iter()
                .min_by(|&&a, &&b| {
                    let da = distance(&traditions[a].position, &c.centroid, metric);
                    let db = distance(&traditions[b].position, &c.centroid, metric);
                    da.partial_cmp(&db).unwrap()
                })
                .copied()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::DialPosition;

    fn make_traditions() -> Vec<Tradition> {
        vec![
            Tradition::new("a", DialPosition::new(0.0, 0.0)),
            Tradition::new("b", DialPosition::new(0.1, 0.1)),
            Tradition::new("c", DialPosition::new(5.0, 5.0)),
            Tradition::new("d", DialPosition::new(5.1, 4.9)),
            Tradition::new("e", DialPosition::new(10.0, 10.0)),
            Tradition::new("f", DialPosition::new(10.1, 10.1)),
        ]
    }

    #[test]
    fn test_kmeans_two_clusters() {
        let traditions = make_traditions();
        let result = kmeans(&traditions, 2, 100, &DistanceMetric::Euclidean);
        assert!(result.converged || result.iterations <= 100);
        // Each cluster should have some members
        assert!(result.clusters.iter().all(|c| !c.members.is_empty()));
    }

    #[test]
    fn test_kmeans_correct_grouping() {
        let traditions = make_traditions();
        let result = kmeans(&traditions, 3, 100, &DistanceMetric::Euclidean);
        // Traditions 0,1 should be in same cluster; 2,3 same; 4,5 same
        let assignments: Vec<usize> = (0..6)
            .map(|i| {
                result
                    .clusters
                    .iter()
                    .position(|c| c.members.contains(&i))
                    .unwrap()
            })
            .collect();
        assert_eq!(assignments[0], assignments[1], "traditions 0 and 1 should cluster together");
        assert_eq!(assignments[2], assignments[3], "traditions 2 and 3 should cluster together");
        assert_eq!(assignments[4], assignments[5], "traditions 4 and 5 should cluster together");
        // All three groups should be in different clusters
        assert_ne!(assignments[0], assignments[2]);
        assert_ne!(assignments[2], assignments[4]);
    }

    #[test]
    fn test_cluster_recompute_centroid() {
        let traditions = vec![
            Tradition::new("a", DialPosition::new(0.0, 0.0)),
            Tradition::new("b", DialPosition::new(2.0, 2.0)),
        ];
        let mut c = Cluster::new(DialPosition::center());
        c.members = vec![0, 1];
        c.recompute_centroid(&traditions);
        assert!((c.centroid.x - 1.0).abs() < 1e-10);
        assert!((c.centroid.y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_elbow_analysis() {
        let traditions = make_traditions();
        let results = elbow_analysis(&traditions, 4, &DistanceMetric::Euclidean);
        assert_eq!(results.len(), 4);
        // Inertia should decrease with more clusters
        for i in 1..results.len() {
            assert!(results[i].1 <= results[i - 1].1 + 1e-6);
        }
    }

    #[test]
    fn test_cluster_representatives() {
        let traditions = make_traditions();
        let result = kmeans(&traditions, 2, 100, &DistanceMetric::Euclidean);
        let reps = cluster_representatives(&traditions, &result.clusters, &DistanceMetric::Euclidean);
        assert_eq!(reps.len(), 2);
        for rep in &reps {
            assert!(rep.is_some());
        }
    }

    #[test]
    fn test_compute_inertia() {
        let traditions = vec![
            Tradition::new("a", DialPosition::new(0.0, 0.0)),
            Tradition::new("b", DialPosition::new(0.0, 0.0)),
        ];
        let mut c = Cluster::new(DialPosition::new(0.0, 0.0));
        c.members = vec![0, 1];
        let inertia = compute_inertia(&traditions, &[c], &DistanceMetric::Euclidean);
        assert!(inertia.abs() < 1e-10);
    }
}
