use std::f64::consts::PI;

use crate::core::{AerodynamicBody, FluidEnvironment};

/// Induced drag (drag due to lift) using the Oswald efficiency model:
///
/// `Cd_i = Cl² / (π · AR · e)`
///
/// `Cl` is back-calculated from the lift force: `Cl = F_l / (q · A_ref)`
///
/// Only meaningful for lifting surfaces (aircraft wings). Negligible for cars.
pub fn calculate_induced_drag(
    body: &dyn AerodynamicBody,
    env: &FluidEnvironment,
    velocity: f64,
    lift_n: f64,
) -> f64 {
    let q = 0.5 * env.air_density * velocity.powi(2);
    let cl = if q * body.reference_area() > f64::EPSILON {
        lift_n / (q * body.reference_area())
    } else {
        0.0
    };
    let cd_induced = cl.powi(2) / (PI * body.aspect_ratio() * body.oswald_efficiency());
    q * cd_induced * body.reference_area()
}
