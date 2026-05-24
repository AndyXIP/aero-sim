use serde::Serialize;

/// One data point produced by a velocity or AoA sweep.
#[derive(Debug, Serialize)]
pub struct SimResult {
    /// Vehicle speed [m/s].
    pub velocity_ms: f64,
    /// Angle of attack [degrees]. Constant in velocity sweeps; varies in AoA sweeps.
    pub aoa_deg: f64,
    /// Parasite (form + friction) aerodynamic drag [N].
    pub aero_drag_n: f64,
    /// Aircraft only: induced drag (drag due to lift) [N].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub induced_drag_n: Option<f64>,
    /// Lift force [N]. Negative = downforce.
    pub lift_n: f64,
    /// Parasite drag-to-lift ratio. `inf` when lift is zero.
    pub drag_to_lift: f64,
    /// Aircraft only: `true` when lift ≥ weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub airborne: Option<bool>,
    /// Boats only: hydrodynamic hull resistance [N].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hull_drag_n: Option<f64>,
    /// Total resistive force — parasite drag + induced drag + hull drag [N].
    pub total_drag_n: f64,
}
