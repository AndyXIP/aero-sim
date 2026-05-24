use crate::core::AerodynamicBody;

/// Hydrodynamic hull resistance for a marine vessel:
///
/// `R_hull = 0.5 * ρ_water * v² * Cd_hull * A_wetted`
///
/// Returns `None` if the body has no hull drag parameters configured.
pub fn calculate_hull_drag(
    body: &dyn AerodynamicBody,
    water_density: f64,
    velocity: f64,
) -> Option<f64> {
    let cd_hull = body.hull_drag_coefficient()?;
    let a_wetted = body.wetted_area_m2()?;
    Some(0.5 * water_density * velocity.powi(2) * cd_hull * a_wetted)
}
