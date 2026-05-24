use serde::Serialize;

/// One data point produced by the velocity sweep.
#[derive(Debug, Serialize)]
pub struct SimResult {
    /// Vehicle speed [m/s].
    pub velocity_ms: f64,
    /// Aerodynamic drag force [N].
    pub aero_drag_n: f64,
    /// Lift force [N]. Negative = downforce.
    pub lift_n: f64,
    /// Aero drag-to-lift ratio. `inf` when lift is zero.
    pub drag_to_lift: f64,
    /// Aircraft only: `true` when lift ≥ weight (body can sustain flight).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub airborne: Option<bool>,
    /// Boats only: hydrodynamic hull resistance [N].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hull_drag_n: Option<f64>,
    /// Total resistive force — aero drag + hull drag where applicable [N].
    pub total_drag_n: f64,
}
