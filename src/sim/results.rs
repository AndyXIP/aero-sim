use serde::Serialize;

/// One data point produced by the velocity sweep.
#[derive(Debug, Serialize)]
pub struct SimResult {
    /// Vehicle speed [m/s].
    pub velocity_ms: f64,
    /// Aerodynamic drag force [N].
    pub drag_n: f64,
    /// Aerodynamic lift force [N]. Negative = downforce.
    pub lift_n: f64,
    /// Drag-to-lift ratio. `inf` when lift is zero.
    pub drag_to_lift: f64,
}
