use std::path::Path;

use serde::Deserialize;

use crate::{core::AerodynamicBody, error::AeroError};

// ── Vehicle ───────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct VehicleConfig {
    pub name: String,
    /// Total mass including driver [kg].
    pub mass_kg: f64,
    /// Drag coefficient (Cd).
    pub drag_coefficient: f64,
    /// Lift coefficient (Cl). Negative = downforce.
    pub lift_coefficient: f64,
    /// Frontal projected area [m²].
    pub frontal_area_m2: f64,
    /// Reference (planform) area for lift [m²].
    pub reference_area_m2: f64,
}

impl AerodynamicBody for VehicleConfig {
    fn drag_coefficient(&self) -> f64 { self.drag_coefficient }
    fn lift_coefficient(&self) -> f64 { self.lift_coefficient }
    fn frontal_area(&self) -> f64 { self.frontal_area_m2 }
    fn reference_area(&self) -> f64 { self.reference_area_m2 }
    fn mass(&self) -> f64 { self.mass_kg }
}

// ── Environment ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct EnvironmentConfig {
    /// Air density [kg/m³].
    pub air_density_kg_m3: f64,
    /// Ambient temperature [°C].
    pub temperature_c: f64,
    /// Headwind speed (positive = into vehicle) [m/s].
    pub wind_speed_ms: f64,
}

// ── Simulation sweep ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SimulationConfig {
    pub velocity_start_ms: f64,
    pub velocity_end_ms: f64,
    pub velocity_step_ms: f64,
    /// Angle of attack [degrees].
    pub angle_of_attack_deg: f64,
    /// Filename written inside the `output/` directory.
    pub output_file: String,
}

// ── Top-level ─────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct Config {
    pub vehicle: VehicleConfig,
    pub environment: EnvironmentConfig,
    pub simulation: SimulationConfig,
}

pub fn load_config(path: impl AsRef<Path>) -> Result<Config, AeroError> {
    let contents = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
