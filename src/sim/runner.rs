use crate::{
    aero::{
        calculate_drag, calculate_hull_drag, calculate_induced_drag, calculate_lift,
        calculate_lift_aircraft,
    },
    core::{AerodynamicBody, FluidEnvironment, VehicleType},
    error::AeroError,
    sim::results::SimResult,
};

// ── Velocity sweep ────────────────────────────────────────────────────────────

/// Sweep velocity at a fixed angle of attack.
pub struct SweepConfig {
    pub vehicle_type: VehicleType,
    pub velocity_start: f64,
    pub velocity_end: f64,
    pub velocity_step: f64,
    /// Fixed angle of attack [degrees]. Used by aircraft; ignored for cars.
    pub angle_of_attack: f64,
    /// Water density [kg/m³]. Used for boats. Seawater ≈ 1025, freshwater ≈ 1000.
    pub water_density: f64,
}

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
        results.push(compute_point(
            body,
            env,
            v,
            config.angle_of_attack,
            config.vehicle_type,
            config.water_density,
            weight_n,
        ));
        v += config.velocity_step;
    }

    Ok(results)
}

// ── AoA sweep ─────────────────────────────────────────────────────────────────

/// Sweep angle of attack at a fixed velocity.
pub struct AoaSweepConfig {
    pub vehicle_type: VehicleType,
    /// Fixed vehicle speed [m/s].
    pub velocity: f64,
    pub aoa_start: f64,
    pub aoa_end: f64,
    pub aoa_step: f64,
    pub water_density: f64,
}

pub fn run_aoa_sweep(
    body: &dyn AerodynamicBody,
    env: &FluidEnvironment,
    config: &AoaSweepConfig,
) -> Result<Vec<SimResult>, AeroError> {
    if config.aoa_step <= 0.0 {
        return Err(AeroError::Simulation(
            "aoa_step must be greater than zero".into(),
        ));
    }

    let weight_n = body.mass() * 9.81;
    let mut results = Vec::new();
    let mut aoa = config.aoa_start;

    while aoa <= config.aoa_end + f64::EPSILON {
        results.push(compute_point(
            body,
            env,
            config.velocity,
            aoa,
            config.vehicle_type,
            config.water_density,
            weight_n,
        ));
        aoa += config.aoa_step;
    }

    Ok(results)
}

// ── Shared point calculation ──────────────────────────────────────────────────

fn compute_point(
    body: &dyn AerodynamicBody,
    env: &FluidEnvironment,
    velocity: f64,
    aoa_deg: f64,
    vehicle_type: VehicleType,
    water_density: f64,
    weight_n: f64,
) -> SimResult {
    let aero_drag = calculate_drag(body, env, velocity);

    let (lift, induced_drag, airborne, hull_drag) = match vehicle_type {
        VehicleType::Car => {
            let lift = calculate_lift(body, env, velocity, aoa_deg);
            (lift, None, None, None)
        }
        VehicleType::Aircraft => {
            let lift = calculate_lift_aircraft(body, env, velocity, aoa_deg);
            let induced = calculate_induced_drag(body, env, velocity, lift);
            (lift, Some(induced), Some(lift >= weight_n), None)
        }
        VehicleType::Boat => {
            let lift = calculate_lift(body, env, velocity, aoa_deg);
            let hull = calculate_hull_drag(body, water_density, velocity);
            (lift, None, None, hull)
        }
    };

    let total_drag = aero_drag + induced_drag.unwrap_or(0.0) + hull_drag.unwrap_or(0.0);
    let drag_to_lift = if lift.abs() > f64::EPSILON {
        aero_drag / lift
    } else {
        f64::INFINITY
    };

    SimResult {
        velocity_ms: velocity,
        aoa_deg,
        aero_drag_n: aero_drag,
        induced_drag_n: induced_drag,
        lift_n: lift,
        drag_to_lift,
        airborne,
        hull_drag_n: hull_drag,
        total_drag_n: total_drag,
    }
}
