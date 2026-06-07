//! Distance metrics between traditions.
//!
//! Provides various distance and similarity metrics for comparing
//! cultural traditions on the dial.

use crate::position::DialPosition;
use crate::tradition::Tradition;

/// Available distance metrics.
#[derive(Debug, Clone, PartialEq)]
pub enum DistanceMetric {
    /// Standard Euclidean distance.
    Euclidean,
    /// Manhattan (L1) distance.
    Manhattan,
    /// Chebyshev (L∞) distance: max of coordinate differences.
    Chebyshev,
    /// Angular distance (angle between position vectors).
    Angular,
    /// Cosine similarity (1 - cosine similarity).
    Cosine,
}

/// Compute distance between two positions using the given metric.
pub fn distance(a: &DialPosition, b: &DialPosition, metric: &DistanceMetric) -> f64 {
    match metric {
        DistanceMetric::Euclidean => a.distance_to(b),
        DistanceMetric::Manhattan => (a.x - b.x).abs() + (a.y - b.y).abs(),
        DistanceMetric::Chebyshev => (a.x - b.x).abs().max((a.y - b.y).abs()),
        DistanceMetric::Angular => {
            let angle_a = a.angle();
            let angle_b = b.angle();
            let diff = (angle_a - angle_b).abs();
            diff.min(2.0 * std::f64::consts::PI - diff)
        }
        DistanceMetric::Cosine => {
            let dot = a.x * b.x + a.y * b.y;
            let mag_a = a.magnitude();
            let mag_b = b.magnitude();
            if mag_a < 1e-10 || mag_b < 1e-10 {
                return 1.0; // undefined → max distance
            }
            1.0 - dot / (mag_a * mag_b)
        }
    }
}

/// Compute distance between two traditions.
pub fn tradition_distance(a: &Tradition, b: &Tradition, metric: &DistanceMetric) -> f64 {
    distance(&a.position, &b.position, metric)
}

/// Compute a full pairwise distance matrix for a set of positions.
pub fn distance_matrix(positions: &[DialPosition], metric: &DistanceMetric) -> Vec<Vec<f64>> {
    let n = positions.len();
    let mut matrix = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let d = distance(&positions[i], &positions[j], metric);
            matrix[i][j] = d;
            matrix[j][i] = d;
        }
    }
    matrix
}

/// Find the two closest positions.
pub fn closest_pair(positions: &[DialPosition], metric: &DistanceMetric) -> (usize, usize, f64) {
    assert!(positions.len() >= 2, "need at least 2 positions");
    let mut best_i = 0;
    let mut best_j = 1;
    let mut best_d = distance(&positions[0], &positions[1], metric);

    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let d = distance(&positions[i], &positions[j], metric);
            if d < best_d {
                best_d = d;
                best_i = i;
                best_j = j;
            }
        }
    }

    (best_i, best_j, best_d)
}

/// Compute the average distance from a position to all others.
pub fn average_distance(pos: &DialPosition, others: &[DialPosition], metric: &DistanceMetric) -> f64 {
    if others.is_empty() {
        return 0.0;
    }
    others.iter().map(|p| distance(pos, p, metric)).sum::<f64>() / others.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(x: f64, y: f64) -> DialPosition {
        DialPosition::new(x, y)
    }

    #[test]
    fn test_euclidean_distance() {
        let d = distance(&p(0.0, 0.0), &p(3.0, 4.0), &DistanceMetric::Euclidean);
        assert!((d - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_manhattan_distance() {
        let d = distance(&p(0.0, 0.0), &p(3.0, 4.0), &DistanceMetric::Manhattan);
        assert!((d - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_chebyshev_distance() {
        let d = distance(&p(0.0, 0.0), &p(3.0, 4.0), &DistanceMetric::Chebyshev);
        assert!((d - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let d = distance(&p(1.0, 0.0), &p(2.0, 0.0), &DistanceMetric::Cosine);
        assert!(d.abs() < 1e-10);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let d = distance(&p(1.0, 0.0), &p(-1.0, 0.0), &DistanceMetric::Cosine);
        assert!((d - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_distance_matrix_symmetric() {
        let positions = vec![p(0.0, 0.0), p(1.0, 0.0), p(0.0, 1.0)];
        let m = distance_matrix(&positions, &DistanceMetric::Euclidean);
        assert_eq!(m.len(), 3);
        for i in 0..3 {
            assert!((m[i][i]).abs() < 1e-10);
            for j in 0..3 {
                assert!((m[i][j] - m[j][i]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn test_closest_pair() {
        let positions = vec![p(0.0, 0.0), p(10.0, 10.0), p(0.1, 0.1)];
        let (i, j, d) = closest_pair(&positions, &DistanceMetric::Euclidean);
        assert!((d - 0.1_f64.mul_add(0.1, 0.01_f64).sqrt()).abs() < 1e-10);
        assert_eq!(i, 0);
        assert_eq!(j, 2);
    }

    #[test]
    fn test_average_distance() {
        let avg = average_distance(
            &p(0.0, 0.0),
            &[p(1.0, 0.0), p(-1.0, 0.0)],
            &DistanceMetric::Euclidean,
        );
        assert!((avg - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tradition_distance() {
        let a = crate::tradition::Tradition::new("a", p(0.0, 0.0));
        let b = crate::tradition::Tradition::new("b", p(3.0, 4.0));
        let d = tradition_distance(&a, &b, &DistanceMetric::Euclidean);
        assert!((d - 5.0).abs() < 1e-10);
    }
}
