//! Cultural traditions with positions on a 2D dial.
//!
//! A tradition represents a cultural orientation with a position
//! on one or more dials.

use crate::position::DialPosition;

/// A cultural tradition with a name and dial positions.
#[derive(Debug, Clone)]
pub struct Tradition {
    /// Name of the tradition.
    pub name: String,
    /// Position on the primary dial.
    pub position: DialPosition,
    /// Strength/weight of adherence to this tradition (0-1).
    pub strength: f64,
    /// Optional description.
    pub description: String,
}

impl Tradition {
    /// Create a new tradition.
    pub fn new(name: &str, position: DialPosition) -> Self {
        Tradition {
            name: name.to_string(),
            position,
            strength: 1.0,
            description: String::new(),
        }
    }

    /// Create a tradition with a strength value.
    pub fn with_strength(name: &str, position: DialPosition, strength: f64) -> Self {
        Tradition {
            name: name.to_string(),
            position,
            strength: strength.clamp(0.0, 1.0),
            description: String::new(),
        }
    }

    /// Set the description.
    pub fn describe(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Distance to another tradition's position.
    pub fn distance_to(&self, other: &Tradition) -> f64 {
        self.position.distance_to(&other.position)
    }

    /// Check if this tradition is similar to another (within threshold).
    pub fn is_similar(&self, other: &Tradition, threshold: f64) -> bool {
        self.distance_to(other) < threshold
    }

    /// Compute a blended position with another tradition, weighted by strengths.
    pub fn blend(&self, other: &Tradition) -> DialPosition {
        let total_strength = self.strength + other.strength;
        if total_strength < 1e-10 {
            return DialPosition::center();
        }
        let w1 = self.strength / total_strength;
        DialPosition::new(
            self.position.x * w1 + other.position.x * (1.0 - w1),
            self.position.y * w1 + other.position.y * (1.0 - w1),
        )
    }
}

/// A collection of traditions forming a cultural landscape.
#[derive(Debug, Clone)]
pub struct TraditionSet {
    /// The traditions in this set.
    pub traditions: Vec<Tradition>,
}

impl TraditionSet {
    /// Create an empty tradition set.
    pub fn new() -> Self {
        TraditionSet { traditions: Vec::new() }
    }

    /// Create from a vector of traditions.
    pub fn from_vec(traditions: Vec<Tradition>) -> Self {
        TraditionSet { traditions }
    }

    /// Add a tradition.
    pub fn add(&mut self, tradition: Tradition) {
        self.traditions.push(tradition);
    }

    /// Find the tradition closest to a given position.
    pub fn nearest(&self, pos: &DialPosition) -> Option<&Tradition> {
        self.traditions
            .iter()
            .min_by(|a, b| {
                a.position
                    .distance_to(pos)
                    .partial_cmp(&b.position.distance_to(pos))
                    .unwrap()
            })
    }

    /// Compute the centroid (average position) of all traditions.
    pub fn centroid(&self) -> DialPosition {
        if self.traditions.is_empty() {
            return DialPosition::center();
        }
        let n = self.traditions.len() as f64;
        let x: f64 = self.traditions.iter().map(|t| t.position.x).sum::<f64>() / n;
        let y: f64 = self.traditions.iter().map(|t| t.position.y).sum::<f64>() / n;
        DialPosition::new(x, y)
    }

    /// Find traditions within a radius of a position.
    pub fn within_radius(&self, pos: &DialPosition, radius: f64) -> Vec<&Tradition> {
        self.traditions
            .iter()
            .filter(|t| t.position.distance_to(pos) < radius)
            .collect()
    }

    /// Number of traditions.
    pub fn len(&self) -> usize {
        self.traditions.len()
    }

    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.traditions.is_empty()
    }
}

impl Default for TraditionSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tradition_creation() {
        let t = Tradition::new("stoicism", DialPosition::new(0.8, -0.3));
        assert_eq!(t.name, "stoicism");
        assert!((t.strength - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tradition_with_strength() {
        let t = Tradition::with_strength("epicurean", DialPosition::center(), 0.5);
        assert!((t.strength - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_tradition_distance() {
        let a = Tradition::new("a", DialPosition::new(0.0, 0.0));
        let b = Tradition::new("b", DialPosition::new(3.0, 4.0));
        assert!((a.distance_to(&b) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_tradition_similarity() {
        let a = Tradition::new("a", DialPosition::new(0.0, 0.0));
        let b = Tradition::new("b", DialPosition::new(0.1, 0.1));
        assert!(a.is_similar(&b, 1.0));
        assert!(!a.is_similar(&b, 0.1));
    }

    #[test]
    fn test_tradition_blend_equal() {
        let a = Tradition::new("a", DialPosition::new(0.0, 0.0));
        let b = Tradition::new("b", DialPosition::new(2.0, 2.0));
        let blend = a.blend(&b);
        assert!((blend.x - 1.0).abs() < 1e-10);
        assert!((blend.y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tradition_blend_weighted() {
        let a = Tradition::with_strength("a", DialPosition::new(0.0, 0.0), 0.75);
        let b = Tradition::with_strength("b", DialPosition::new(4.0, 0.0), 0.25);
        let blend = a.blend(&b);
        assert!((blend.x - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tradition_set_nearest() {
        let mut ts = TraditionSet::new();
        ts.add(Tradition::new("a", DialPosition::new(0.0, 0.0)));
        ts.add(Tradition::new("b", DialPosition::new(5.0, 5.0)));
        let nearest = ts.nearest(&DialPosition::new(4.0, 4.0));
        assert!(nearest.is_some());
        assert_eq!(nearest.unwrap().name, "b");
    }

    #[test]
    fn test_tradition_set_centroid() {
        let ts = TraditionSet::from_vec(vec![
            Tradition::new("a", DialPosition::new(0.0, 0.0)),
            Tradition::new("b", DialPosition::new(2.0, 4.0)),
        ]);
        let c = ts.centroid();
        assert!((c.x - 1.0).abs() < 1e-10);
        assert!((c.y - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_tradition_set_within_radius() {
        let ts = TraditionSet::from_vec(vec![
            Tradition::new("a", DialPosition::new(0.0, 0.0)),
            Tradition::new("b", DialPosition::new(1.0, 0.0)),
            Tradition::new("c", DialPosition::new(5.0, 5.0)),
        ]);
        let within = ts.within_radius(&DialPosition::center(), 1.5);
        assert_eq!(within.len(), 2);
    }

    #[test]
    fn test_tradition_describe() {
        let t = Tradition::new("test", DialPosition::center()).describe("A test tradition");
        assert_eq!(t.description, "A test tradition");
    }
}
