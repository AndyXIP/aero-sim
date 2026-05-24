use crate::core::{AerodynamicBody, FluidEnvironment};

/// Aerodynamic lift force using the standard lift equation:
///
/// `F_l = 0.5 * ρ * v² * Cl * A_ref`
///
/// A negative result indicates downforce. The `aoa_deg` parameter (angle of
/// attack in degrees) is available for AoA-dependent Cl models — extend this
/// function when needed.
pub fn calculate_lift(
    body: &dyn AerodynamicBody,
    env: &FluidEnvironment,
    velocity: f64,
    _aoa_deg: f64,
) -> f64 {
    0.5 * env.air_density * velocity.powi(2) * body.lift_coefficient() * body.reference_area()
}
