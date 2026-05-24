use serde::Deserialize;

/// Selects the aerodynamic and hydrodynamic model used during simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VehicleType {
    /// Ground vehicle. Uses fixed Cl; AoA is ignored.
    Car,
    /// Fixed-wing aircraft. Cl varies with AoA; stall is modelled.
    Aircraft,
    /// Marine vessel. Adds hydrodynamic hull drag alongside aero drag.
    Boat,
}
