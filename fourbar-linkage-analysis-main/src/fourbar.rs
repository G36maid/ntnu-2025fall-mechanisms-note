//! Four-Bar Linkage Mechanism Core Logic
//!
//! This module implements the Newton-Raphson numerical solver for
//! four-bar linkage position analysis.

/// Configuration parameters for a four-bar linkage
#[derive(Debug, Clone, Copy)]
pub struct FourBarConfig {
    pub r1: f64, // Ground link length
    pub r2: f64, // Input crank length
    pub r3: f64, // Coupler link length
    pub r4: f64, // Output rocker length
}

impl Default for FourBarConfig {
    fn default() -> Self {
        Self {
            r1: 6.0,
            r2: 2.0,
            r3: 5.0,
            r4: 5.0,
        }
    }
}

/// State of the four-bar linkage
#[derive(Debug, Clone, Copy)]
pub struct FourBarState {
    pub theta2: f64, // Input angle (radians)
    pub theta3: f64, // Coupler angle (radians)
    pub theta4: f64, // Output angle (radians)
}

impl Default for FourBarState {
    fn default() -> Self {
        Self {
            theta2: 0.0,
            theta3: 0.0,
            theta4: 0.0,
        }
    }
}

/// Four-bar linkage mechanism solver
pub struct FourBar {
    pub config: FourBarConfig,
    pub state: FourBarState,
    tolerance: f64,
    max_iterations: usize,
}

impl FourBar {
    /// Create a new four-bar linkage with default configuration
    pub fn new() -> Self {
        Self {
            config: FourBarConfig::default(),
            state: FourBarState::default(),
            tolerance: 1e-9,
            max_iterations: 100,
        }
    }

    /// Create with custom configuration
    #[allow(dead_code)]
    pub fn with_config(config: FourBarConfig) -> Self {
        Self {
            config,
            state: FourBarState::default(),
            tolerance: 1e-9,
            max_iterations: 100,
        }
    }

    /// Update input angle (in degrees) and solve for theta3 and theta4
    pub fn set_theta2_degrees(&mut self, theta2_deg: f64) -> Result<(), String> {
        self.state.theta2 = theta2_deg.to_radians();
        self.solve()
    }

    /// Solve for theta3 and theta4 using Newton-Raphson method
    /// Uses the previous state as initial guess for continuity
    pub fn solve(&mut self) -> Result<(), String> {
        let FourBarConfig { r1, r2, r3, r4 } = self.config;
        let theta2 = self.state.theta2;

        // Use previous solution as initial guess (or analytical solution if first time)
        let mut theta3 = self.state.theta3;
        let mut theta4 = self.state.theta4;

        // If this is the first solve or angles are zero, use analytical solution for initial guess
        if theta3.abs() < 1e-6 && theta4.abs() < 1e-6 {
            if let Some((t3, t4)) = self.analytical_solution(theta2) {
                theta3 = t3;
                theta4 = t4;
            } else {
                // Fallback to geometric guess
                theta3 = 45f64.to_radians();
                theta4 = 45f64.to_radians();
            }
        }

        // Newton-Raphson iteration
        for _iter in 0..self.max_iterations {
            // Compute error functions
            let f1 = r2 * theta2.cos() + r3 * theta3.cos() - r4 * theta4.cos() - r1;
            let f2 = r2 * theta2.sin() + r3 * theta3.sin() - r4 * theta4.sin();

            // Check convergence
            if f1.abs() < self.tolerance && f2.abs() < self.tolerance {
                self.state.theta3 = theta3;
                self.state.theta4 = theta4;
                return Ok(());
            }

            // Compute Jacobian matrix
            let j11 = -r3 * theta3.sin();
            let j12 = r4 * theta4.sin();
            let j21 = r3 * theta3.cos();
            let j22 = -r4 * theta4.cos();

            // Check for singularity
            let det = j11 * j22 - j12 * j21;
            if det.abs() < 1e-9 {
                return Err(format!(
                    "Singularity detected at theta2 = {:.1}°",
                    theta2.to_degrees()
                ));
            }

            // Solve linear system: J * delta = -F
            let d_theta3 = (-f1 * j22 + f2 * j12) / det;
            let d_theta4 = (j11 * (-f2) - j21 * (-f1)) / det;

            // Update guess
            theta3 += d_theta3;
            theta4 += d_theta4;
        }

        Err(format!(
            "Max iterations ({}) reached at theta2 = {:.1}°",
            self.max_iterations,
            theta2.to_degrees()
        ))
    }

    /// Analytical solution for initial guess (Open configuration)
    fn analytical_solution(&self, theta2: f64) -> Option<(f64, f64)> {
        let FourBarConfig { r1, r2, r3, r4 } = self.config;

        let k1 = r1 / r2;
        let k2 = r1 / r4;
        let k3 = (r1.powi(2) + r2.powi(2) + r3.powi(2) - r4.powi(2)) / (2.0 * r2 * r3);

        let a = theta2.cos() - k1 - k2 * theta2.cos() + k3;
        let b = -2.0 * theta2.sin();
        let c = k1 - (k2 + 1.0) * theta2.cos() + k3;

        // Solve for theta4 using half-angle substitution
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Open configuration (typically the one we want)
        let theta4 = 2.0 * (((-b + discriminant.sqrt()) / (2.0 * a)).atan());

        // Solve for theta3
        let k4 = r1 / r3;
        let k5 = (r4.powi(2) - r1.powi(2) - r2.powi(2) - r3.powi(2)) / (2.0 * r2 * r3);

        let d = theta2.cos() - k1 + k4 * theta4.cos() + k5;
        let e = -2.0 * theta2.sin();
        let f = k1 + (k4 - 1.0) * theta4.cos() + k5;

        let discriminant2 = e.powi(2) - 4.0 * d * f;
        if discriminant2 < 0.0 {
            return None;
        }

        let theta3 = 2.0 * (((-e - discriminant2.sqrt()) / (2.0 * d)).atan());

        Some((theta3, theta4))
    }

    /// Get joint positions in Cartesian coordinates
    pub fn get_positions(&self) -> JointPositions {
        let FourBarConfig { r1, r2, r3, r4: _ } = self.config;
        let FourBarState {
            theta2,
            theta3,
            theta4: _,
        } = self.state;

        JointPositions {
            p1: Point2D { x: 0.0, y: 0.0 }, // Fixed ground joint
            p2: Point2D {
                x: r2 * theta2.cos(),
                y: r2 * theta2.sin(),
            }, // End of input crank
            p3: Point2D {
                x: r2 * theta2.cos() + r3 * theta3.cos(),
                y: r2 * theta2.sin() + r3 * theta3.sin(),
            }, // Coupler point
            p4: Point2D { x: r1, y: 0.0 },  // Fixed ground joint
        }
    }

    /// Check if mechanism satisfies Grashof condition
    pub fn is_grashof(&self) -> bool {
        let FourBarConfig { r1, r2, r3, r4 } = self.config;
        let lengths = [r1, r2, r3, r4];
        let s = lengths.iter().cloned().fold(f64::INFINITY, f64::min);
        let l = lengths.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let p_q_sum: f64 = lengths.iter().sum::<f64>() - s - l;

        s + l <= p_q_sum
    }

    /// Get mechanism type description
    pub fn mechanism_type(&self) -> &str {
        if !self.is_grashof() {
            return "Non-Grashof (Double Rocker)";
        }

        let FourBarConfig { r1, r2, r3, r4 } = self.config;
        let lengths = [
            (r1, "Ground"),
            (r2, "Crank"),
            (r3, "Coupler"),
            (r4, "Rocker"),
        ];
        let shortest = lengths
            .iter()
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap();

        match shortest.1 {
            "Crank" => "Crank-Rocker",
            "Ground" => "Double Crank",
            "Coupler" => "Double Rocker",
            "Rocker" => "Rocker-Crank",
            _ => "Unknown",
        }
    }
}

impl Default for FourBar {
    fn default() -> Self {
        Self::new()
    }
}

/// 2D Point
#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

/// Joint positions of the four-bar linkage
#[derive(Debug, Clone, Copy)]
pub struct JointPositions {
    pub p1: Point2D, // Ground joint 1 (origin)
    pub p2: Point2D, // End of input crank
    pub p3: Point2D, // Coupler point
    pub p4: Point2D, // Ground joint 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mechanism() {
        let mut linkage = FourBar::new();
        assert!(linkage.set_theta2_degrees(45.0).is_ok());
    }

    #[test]
    fn test_grashof_condition() {
        let linkage = FourBar::new();
        assert!(linkage.is_grashof());
    }

    #[test]
    fn test_full_rotation() {
        let mut linkage = FourBar::new();
        for theta2 in (0..360).step_by(10) {
            let result = linkage.set_theta2_degrees(theta2 as f64);
            assert!(result.is_ok(), "Failed at theta2 = {}°", theta2);
        }
    }
}
