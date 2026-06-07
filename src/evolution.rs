//! How traditions evolve over time.
//!
//! Models tradition drift, diffusion, merging, and splitting
//! as agents interact and their cultural positions shift.

use crate::position::DialPosition;
use crate::tradition::Tradition;

/// How a tradition evolves in one step.
#[derive(Debug, Clone)]
pub enum EvolutionEvent {
    /// Tradition drifts toward a target position.
    Drift {
        /// Target position.
        target: DialPosition,
        /// Rate of drift (0-1).
        rate: f64,
    },
    /// Two traditions merge into one.
    Merge {
        /// Index of the other tradition.
        other_idx: usize,
        /// Weight of this tradition in the merge (0-1).
        weight: f64,
    },
    /// A tradition splits into two.
    Split {
        /// Offset for the new tradition.
        offset: DialPosition,
    },
    /// Tradition adapts toward a pressure point.
    Pressure {
        /// External pressure point.
        source: DialPosition,
        /// Strength of the pressure.
        strength: f64,
    },
}

/// Apply an evolution event to a tradition, returning the new position.
pub fn apply_event(tradition: &Tradition, event: &EvolutionEvent) -> DialPosition {
    match event {
        EvolutionEvent::Drift { target, rate } => {
            tradition.position.move_toward(target, *rate)
        }
        EvolutionEvent::Merge { other_idx: _, weight } => {
            // In practice, the other tradition would be looked up.
            // Here we just return a weighted shift toward center.
            DialPosition::new(
                tradition.position.x * weight,
                tradition.position.y * weight,
            )
        }
        EvolutionEvent::Split { offset } => {
            DialPosition::new(
                tradition.position.x + offset.x,
                tradition.position.y + offset.y,
            )
        }
        EvolutionEvent::Pressure { source, strength } => {
            tradition.position.move_toward(source, *strength)
        }
    }
}

/// A timeline of tradition evolution.
#[derive(Debug, Clone)]
pub struct EvolutionTimeline {
    /// Name of the tradition.
    pub name: String,
    /// Positions at each time step.
    pub positions: Vec<DialPosition>,
    /// Time step labels.
    pub steps: Vec<u64>,
}

impl EvolutionTimeline {
    /// Create a new timeline.
    pub fn new(name: &str, initial: DialPosition) -> Self {
        EvolutionTimeline {
            name: name.to_string(),
            positions: vec![initial],
            steps: vec![0],
        }
    }

    /// Record a new position at a given step.
    pub fn record(&mut self, step: u64, position: DialPosition) {
        self.steps.push(step);
        self.positions.push(position);
    }

    /// Total distance traveled.
    pub fn total_distance(&self) -> f64 {
        if self.positions.len() < 2 {
            return 0.0;
        }
        self.positions
            .windows(2)
            .map(|w| w[0].distance_to(&w[1]))
            .sum()
    }

    /// Average speed (distance per step).
    pub fn average_speed(&self) -> f64 {
        let n_steps = self.steps.len().saturating_sub(1) as f64;
        if n_steps == 0.0 {
            return 0.0;
        }
        self.total_distance() / n_steps
    }

    /// Current position.
    pub fn current(&self) -> &DialPosition {
        self.positions.last().unwrap()
    }

    /// Displacement from start to current.
    pub fn displacement(&self) -> f64 {
        if self.positions.len() < 2 {
            return 0.0;
        }
        self.positions.first().unwrap().distance_to(self.positions.last().unwrap())
    }
}

/// Simulate drift evolution for multiple traditions over N steps.
pub fn simulate_drift(
    traditions: &[Tradition],
    targets: &[DialPosition],
    rate: f64,
    steps: u64,
) -> Vec<EvolutionTimeline> {
    assert_eq!(traditions.len(), targets.len(), "need one target per tradition");

    let mut timelines: Vec<EvolutionTimeline> = traditions
        .iter()
        .map(|t| EvolutionTimeline::new(&t.name, t.position.clone()))
        .collect();

    let mut current_positions: Vec<DialPosition> = traditions.iter().map(|t| t.position.clone()).collect();

    for s in 1..=steps {
        for (i, pos) in current_positions.iter_mut().enumerate() {
            *pos = pos.move_toward(&targets[i], rate);
            timelines[i].record(s, pos.clone());
        }
    }

    timelines
}

/// Simulate convergent evolution — all traditions drift toward a common centroid.
pub fn simulate_convergence(
    traditions: &[Tradition],
    rate: f64,
    steps: u64,
) -> Vec<EvolutionTimeline> {
    // Compute centroid
    let n = traditions.len() as f64;
    let cx: f64 = traditions.iter().map(|t| t.position.x).sum::<f64>() / n;
    let cy: f64 = traditions.iter().map(|t| t.position.y).sum::<f64>() / n;
    let centroid = DialPosition::new(cx, cy);

    let targets: Vec<DialPosition> = (0..traditions.len()).map(|_| centroid.clone()).collect();
    simulate_drift(traditions, &targets, rate, steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drift_event() {
        let t = Tradition::new("test", DialPosition::new(0.0, 0.0));
        let event = EvolutionEvent::Drift {
            target: DialPosition::new(10.0, 0.0),
            rate: 0.5,
        };
        let new_pos = apply_event(&t, &event);
        assert!((new_pos.x - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_pressure_event() {
        let t = Tradition::new("test", DialPosition::new(0.0, 0.0));
        let event = EvolutionEvent::Pressure {
            source: DialPosition::new(1.0, 1.0),
            strength: 0.1,
        };
        let new_pos = apply_event(&t, &event);
        assert!(new_pos.x > 0.0 && new_pos.x < 1.0);
    }

    #[test]
    fn test_split_event() {
        let t = Tradition::new("test", DialPosition::new(5.0, 5.0));
        let event = EvolutionEvent::Split {
            offset: DialPosition::new(1.0, -1.0),
        };
        let new_pos = apply_event(&t, &event);
        assert!((new_pos.x - 6.0).abs() < 1e-10);
        assert!((new_pos.y - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_timeline_total_distance() {
        let mut tl = EvolutionTimeline::new("test", DialPosition::new(0.0, 0.0));
        tl.record(1, DialPosition::new(3.0, 4.0)); // distance 5
        assert!((tl.total_distance() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_timeline_displacement() {
        let mut tl = EvolutionTimeline::new("test", DialPosition::new(0.0, 0.0));
        tl.record(1, DialPosition::new(3.0, 4.0));
        assert!((tl.displacement() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_simulate_drift() {
        let traditions = vec![
            Tradition::new("a", DialPosition::new(0.0, 0.0)),
            Tradition::new("b", DialPosition::new(10.0, 0.0)),
        ];
        let targets = vec![
            DialPosition::new(5.0, 0.0),
            DialPosition::new(5.0, 0.0),
        ];
        let timelines = simulate_drift(&traditions, &targets, 0.5, 10);
        assert_eq!(timelines.len(), 2);
        // Both should have moved toward (5, 0)
        assert!(timelines[0].current().x > 0.0);
        assert!(timelines[1].current().x < 10.0);
    }

    #[test]
    fn test_simulate_convergence() {
        let traditions = vec![
            Tradition::new("a", DialPosition::new(0.0, 0.0)),
            Tradition::new("b", DialPosition::new(10.0, 10.0)),
        ];
        let timelines = simulate_convergence(&traditions, 0.3, 50);
        // Both should be near centroid (5, 5)
        let pos_a = timelines[0].current();
        let pos_b = timelines[1].current();
        assert!((pos_a.x - 5.0).abs() < 1.0);
        assert!((pos_b.x - 5.0).abs() < 1.0);
    }

    #[test]
    fn test_timeline_average_speed() {
        let mut tl = EvolutionTimeline::new("test", DialPosition::new(0.0, 0.0));
        tl.record(1, DialPosition::new(3.0, 4.0)); // 5 distance
        tl.record(2, DialPosition::new(6.0, 8.0)); // 5 distance
        assert!((tl.average_speed() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_timeline_current() {
        let tl = EvolutionTimeline::new("test", DialPosition::new(42.0, 0.0));
        assert!((tl.current().x - 42.0).abs() < 1e-10);
    }
}
