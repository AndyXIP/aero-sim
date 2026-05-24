use crate::{
    aero::{calculate_drag, calculate_lift},
    core::{AerodynamicBody, FluidEnvironment},
    error::AeroError,
    sim::results::SimResult,
};

/// Parameters controlling the velocity sweep.
pub struct SweepConfig {
    /// Start of velocity range [m/s].
    pub velocity_start: f64,
    /// End of velocity range [m/s] (inclusive).
    pub velocity_end: f64,
    /// Step size between velocity samples [m/s].
    pub velocity_step: f64,
    /// Angle of attack [degrees].
    pub angle_of_attack: f64,
}

/// Sweep the velocity range defined by `config`, computing drag and lift at each step.
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

    let mut results = Vec::new();
    let mut v = config.velocity_start;

    while v <= config.velocity_end + f64::EPSILON {
        let drag = calculate_drag(body, env, v);
        let lift = calculate_lift(body, env, v, config.angle_of_attack);
        let drag_to_lift = if lift.abs() > f64::EPSILON {
            drag / lift
        } else {
            f64::INFINITY
        };

        results.push(SimResult {
            velocity_ms: v,
            drag_n: drag,
            lift_n: lift,
            drag_to_lift,
        });

        v += config.velocity_step;
    }

    Ok(results)
}
