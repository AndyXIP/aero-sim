use crate::{
    aero::{calculate_drag, calculate_hull_drag, calculate_lift, calculate_lift_aircraft},
    core::{AerodynamicBody, FluidEnvironment, VehicleType},
    error::AeroError,
    sim::results::SimResult,
};

/// Parameters controlling the velocity sweep.
pub struct SweepConfig {
    pub vehicle_type: VehicleType,
    /// Start of velocity range [m/s].
    pub velocity_start: f64,
    /// End of velocity range [m/s] (inclusive).
    pub velocity_end: f64,
    /// Step size between velocity samples [m/s].
    pub velocity_step: f64,
    /// Angle of attack [degrees]. Used by aircraft model; ignored for cars.
    pub angle_of_attack: f64,
    /// Water density [kg/m³]. Used for boats. Seawater ≈ 1025, freshwater ≈ 1000.
    pub water_density: f64,
}

/// Sweep the velocity range, computing forces at each step.
///
/// Dispatches to the correct physics model based on `config.vehicle_type`:
/// - **Car** — fixed Cl, AoA ignored.
/// - **Aircraft** — Cl(AoA) with stall; records whether lift ≥ weight.
/// - **Boat** — fixed Cl for aero drag + hydrodynamic hull resistance.
pub fn run_sweep(
    body: &dyn AerodynamicBody,
    env: &FluidEnvironment,
    config: &SweepConfig,
) -> Result<Vec<SimResult>, AeroError> {
    if config.velocity_step <= 0.0 {
        return Err(AeroError::Simulation(
            "velocity_step must be greater than zero".into(),
        ));
    }
    if config.velocity_start > config.velocity_end {
        return Err(AeroError::Simulation(
            "velocity_start must be <= velocity_end".into(),
        ));
    }

    let weight_n = body.mass() * 9.81;
    let mut results = Vec::new();
    let mut v = config.velocity_start;

    while v <= config.velocity_end + f64::EPSILON {
        let aero_drag = calculate_drag(body, env, v);

        let (lift, airborne, hull_drag) = match config.vehicle_type {
            VehicleType::Car => (calculate_lift(body, env, v, config.angle_of_attack), None, None),
            VehicleType::Aircraft => {
                let lift = calculate_lift_aircraft(body, env, v, config.angle_of_attack);
                (lift, Some(lift >= weight_n), None)
            }
            VehicleType::Boat => {
                let lift = calculate_lift(body, env, v, config.angle_of_attack);
                let hull = calculate_hull_drag(body, config.water_density, v);
                (lift, None, hull)
            }
        };

        let total_drag = aero_drag + hull_drag.unwrap_or(0.0);
        let drag_to_lift = if lift.abs() > f64::EPSILON {
            aero_drag / lift
        } else {
            f64::INFINITY
        };

        results.push(SimResult {
            velocity_ms: v,
            aero_drag_n: aero_drag,
            lift_n: lift,
            drag_to_lift,
            airborne,
            hull_drag_n: hull_drag,
            total_drag_n: total_drag,
        });

        v += config.velocity_step;
    }

    Ok(results)
}
