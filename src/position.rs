//! DialPosition — a 2D coordinate on a cultural dial.
//!
//! Each dial position represents where an agent (or tradition) falls
//! on a theoretical spectrum, encoded as (x, y) coordinates.

/// A position on a 2D cultural dial.
///
/// Coordinates are typically in [-1, 1] × [-1, 1], where:
/// - (-1, -1) = extreme one end
/// - (0, 0) = neutral / center
/// - (1, 1) = extreme other end
#[derive(Debug, Clone, PartialEq)]
pub struct DialPosition {
    /// X coordinate on the dial.
    pub x: f64,
    /// Y coordinate on the dial.
    pub y: f64,
}

impl DialPosition {
    /// Create a new dial position.
    pub fn new(x: f64, y: f64) -> Self {
        DialPosition { x, y }
    }

    /// The center/neutral position.
    pub fn center() -> Self {
        DialPosition { x: 0.0, y: 0.0 }
    }

    /// Clamp coordinates to [-1, 1].
    pub fn normalized(&self) -> Self {
        DialPosition {
            x: self.x.clamp(-1.0, 1.0),
            y: self.y.clamp(-1.0, 1.0),
        }
    }

    /// Euclidean distance to another position.
    pub fn distance_to(&self, other: &DialPosition) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Midpoint between this position and another.
    pub fn midpoint(&self, other: &DialPosition) -> Self {
        DialPosition {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }

    /// Weighted average of this position with another.
    pub fn weighted_avg(&self, other: &DialPosition, self_weight: f64) -> Self {
        let other_weight = 1.0 - self_weight;
        DialPosition {
            x: self.x * self_weight + other.x * other_weight,
            y: self.y * self_weight + other.y * other_weight,
        }
    }

    /// Angle from origin in radians.
    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    /// Magnitude (distance from origin).
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Move toward a target by a given factor (0 = stay, 1 = arrive).
    pub fn move_toward(&self, target: &DialPosition, factor: f64) -> Self {
        self.weighted_avg(target, 1.0 - factor)
    }

    /// Check if position is within the unit square [-1, 1]².
    pub fn is_valid(&self) -> bool {
        self.x >= -1.0 && self.x <= 1.0 && self.y >= -1.0 && self.y <= 1.0
    }
}

impl std::fmt::Display for DialPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.3}, {:.3})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center() {
        let c = DialPosition::center();
        assert!((c.x).abs() < 1e-10);
        assert!((c.y).abs() < 1e-10);
    }

    #[test]
    fn test_distance() {
        let a = DialPosition::new(0.0, 0.0);
        let b = DialPosition::new(3.0, 4.0);
        assert!((a.distance_to(&b) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_midpoint() {
        let a = DialPosition::new(0.0, 0.0);
        let b = DialPosition::new(2.0, 4.0);
        let m = a.midpoint(&b);
        assert!((m.x - 1.0).abs() < 1e-10);
        assert!((m.y - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_weighted_avg() {
        let a = DialPosition::new(0.0, 0.0);
        let b = DialPosition::new(10.0, 10.0);
        let w = a.weighted_avg(&b, 0.8); // 80% self (a), 20% other (b)
        assert!((w.x - 2.0).abs() < 1e-10);
        assert!((w.y - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_magnitude() {
        let p = DialPosition::new(3.0, 4.0);
        assert!((p.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_normalized() {
        let p = DialPosition::new(5.0, -3.0);
        let n = p.normalized();
        assert!((n.x - 1.0).abs() < 1e-10);
        assert!((n.y - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_move_toward() {
        let a = DialPosition::new(0.0, 0.0);
        let b = DialPosition::new(10.0, 0.0);
        let c = a.move_toward(&b, 0.5);
        assert!((c.x - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_is_valid() {
        assert!(DialPosition::new(0.5, -0.5).is_valid());
        assert!(!DialPosition::new(1.5, 0.0).is_valid());
    }

    #[test]
    fn test_angle() {
        let p = DialPosition::new(1.0, 0.0);
        assert!((p.angle()).abs() < 1e-10);
        let p2 = DialPosition::new(0.0, 1.0);
        assert!((p2.angle() - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
    }

    #[test]
    fn test_display() {
        let p = DialPosition::new(1.234, -5.678);
        let s = format!("{}", p);
        assert!(s.contains("1.234"));
        assert!(s.contains("-5.678"));
    }
}
