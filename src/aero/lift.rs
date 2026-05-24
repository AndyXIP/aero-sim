use std::f64::consts::PI;

use crate::core::{AerodynamicBody, FluidEnvironment};

/// Basic lift using the body's fixed Cl (cars, boats):
///
/// `F_l = 0.5 * ρ * v² * Cl * A_ref`
///
/// AoA is ignored — Cl is treated as a fixed aerodynamic property of the body.
pub fn calculate_lift(
    body: &dyn AerodynamicBody,
    env: &FluidEnvironment,
    velocity: f64,
    _aoa_deg: f64,
) -> f64 {
    0.5 * env.air_density * velocity.powi(2) * body.lift_coefficient() * body.reference_area()
}

/// Aircraft lift with an AoA-dependent Cl model and stall (thin-airfoil theory):
///
/// **Linear region** (`|α| ≤ α_stall`):
/// `Cl = Cl₀ + 2π·sin(α)`
///
/// **Post-stall** (`|α| > α_stall`):
/// `Cl = 2·sin(α)·cos(α)`  (flat-plate approximation — Cl drops sharply)
///
/// `Cl₀` is the body's baseline lift coefficient at 0° AoA (accounts for camber).
pub fn calculate_lift_aircraft(
    body: &dyn AerodynamicBody,
    env: &FluidEnvironment,
    velocity: f64,
    aoa_deg: f64,
) -> f64 {
    let aoa_rad = aoa_deg.to_radians();
    let cl = if aoa_deg.abs() <= body.stall_aoa_deg() {
        body.lift_coefficient() + 2.0 * PI * aoa_rad.sin()
    } else {
        // Post-stall: significant lift loss
        2.0 * aoa_rad.sin() * aoa_rad.cos()
    };
    0.5 * env.air_density * velocity.powi(2) * cl * body.reference_area()
}
