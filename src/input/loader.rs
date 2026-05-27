use std::path::Path;

use serde::Deserialize;

use crate::{
    core::{AerodynamicBody, VehicleType},
    error::AeroError,
};

// ── Vehicle ───────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct VehicleConfig {
    pub name: String,
    /// Total mass including payload / driver [kg].
    pub mass_kg: f64,
    /// Drag coefficient (Cd).
    pub drag_coefficient: f64,
    /// Lift coefficient (Cl). Negative = downforce. Aircraft: baseline Cl at 0° AoA.
    pub lift_coefficient: f64,
    /// Frontal projected area [m²].
    pub frontal_area_m2: f64,
    /// Reference (planform) area for lift [m²].
    pub reference_area_m2: f64,
    /// Critical AoA at which stall begins [degrees]. Aircraft only. Defaults to 16°.
    pub stall_aoa_deg: Option<f64>,
    /// Aspect ratio (b² / A_ref). Aircraft only. Defaults to 8.0.
    pub aspect_ratio: Option<f64>,
    /// Oswald efficiency factor (0–1). Aircraft only. Defaults to 0.8.
    pub oswald_efficiency: Option<f64>,
    /// Hull drag coefficient (Cd_hull). Boats only.
    pub hull_drag_coefficient: Option<f64>,
    /// Wetted hull area [m²]. Boats only.
    pub wetted_area_m2: Option<f64>,
}

impl AerodynamicBody for VehicleConfig {
    fn drag_coefficient(&self) -> f64 {
        self.drag_coefficient
    }
    fn lift_coefficient(&self) -> f64 {
        self.lift_coefficient
    }
    fn frontal_area(&self) -> f64 {
        self.frontal_area_m2
    }
    fn reference_area(&self) -> f64 {
        self.reference_area_m2
    }
    fn mass(&self) -> f64 {
        self.mass_kg
    }
    fn stall_aoa_deg(&self) -> f64 {
        self.stall_aoa_deg.unwrap_or(16.0)
    }
    fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio.unwrap_or(8.0)
    }
    fn oswald_efficiency(&self) -> f64 {
        self.oswald_efficiency.unwrap_or(0.8)
    }
    fn hull_drag_coefficient(&self) -> Option<f64> {
        self.hull_drag_coefficient
    }
    fn wetted_area_m2(&self) -> Option<f64> {
        self.wetted_area_m2
    }
}

// ── Environment ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct EnvironmentConfig {
    pub air_density_kg_m3: f64,
    pub temperature_c: f64,
    pub wind_speed_ms: f64,
    /// Water density [kg/m³]. Boats only. Defaults to 1025 (seawater).
    pub water_density_kg_m3: Option<f64>,
}

// ── Simulation ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SimulationConfig {
    pub vehicle_type: VehicleType,
    // Velocity sweep
    pub velocity_start_ms: f64,
    pub velocity_end_ms: f64,
    pub velocity_step_ms: f64,
    pub angle_of_attack_deg: f64,
    pub output_file: String,
    // AoA sweep (required when --aoa-sweep is passed)
    pub velocity_fixed_ms: Option<f64>,
    pub aoa_start_deg: Option<f64>,
    pub aoa_end_deg: Option<f64>,
    pub aoa_step_deg: Option<f64>,
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
