use crate::core::{AerodynamicBody, FluidEnvironment};

/// Aerodynamic drag force using the standard drag equation:
///
/// `F_d = 0.5 * ρ * v_eff² * Cd * A_frontal`
///
/// where `v_eff` is vehicle speed plus headwind speed.
pub fn calculate_drag(body: &dyn AerodynamicBody, env: &FluidEnvironment, velocity: f64) -> f64 {
    let v_eff = velocity + env.wind_speed;
    0.5 * env.air_density * v_eff.powi(2) * body.drag_coefficient() * body.frontal_area()
}
