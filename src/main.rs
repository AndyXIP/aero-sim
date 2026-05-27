mod aero;
mod core;
mod error;
mod input;
mod sim;

use std::path::PathBuf;

use clap::Parser;

use crate::core::VehicleType;
use error::AeroError;

#[derive(Parser)]
#[command(name = "aero-sim", about = "Aerodynamic force simulator", version)]
struct Cli {
    /// Path to vehicle config TOML
    #[arg(default_value = "examples/f1_car.toml")]
    config: PathBuf,

    /// Output directory
    #[arg(long, short = 'o', default_value = "output")]
    out_dir: PathBuf,

    /// Also write a JSON file alongside the CSV
    #[arg(long)]
    json: bool,

    /// Sweep angle of attack at a fixed velocity instead of sweeping velocity
    #[arg(long)]
    aoa_sweep: bool,
}

fn main() -> Result<(), AeroError> {
    let cli = Cli::parse();

    println!("Loading config: {}", cli.config.display());
    let config = input::load_config(&cli.config)?;
    println!(
        "Vehicle: {} ({:?})",
        config.vehicle.name, config.simulation.vehicle_type
    );

    let env = core::FluidEnvironment {
        air_density: config.environment.air_density_kg_m3,
        temperature: config.environment.temperature_c,
        wind_speed: config.environment.wind_speed_ms,
    };

    let results = if cli.aoa_sweep {
        let aoa_cfg = sim::AoaSweepConfig {
            vehicle_type: config.simulation.vehicle_type,
            velocity: config.simulation.velocity_fixed_ms.ok_or_else(|| {
                AeroError::InvalidConfig("velocity_fixed_ms required for --aoa-sweep".into())
            })?,
            aoa_start: config.simulation.aoa_start_deg.ok_or_else(|| {
                AeroError::InvalidConfig("aoa_start_deg required for --aoa-sweep".into())
            })?,
            aoa_end: config.simulation.aoa_end_deg.ok_or_else(|| {
                AeroError::InvalidConfig("aoa_end_deg required for --aoa-sweep".into())
            })?,
            aoa_step: config.simulation.aoa_step_deg.ok_or_else(|| {
                AeroError::InvalidConfig("aoa_step_deg required for --aoa-sweep".into())
            })?,
            water_density: config.environment.water_density_kg_m3.unwrap_or(1025.0),
        };
        sim::run_aoa_sweep(&config.vehicle, &env, &aoa_cfg)?
    } else {
        let sweep = sim::SweepConfig {
            vehicle_type: config.simulation.vehicle_type,
            velocity_start: config.simulation.velocity_start_ms,
            velocity_end: config.simulation.velocity_end_ms,
            velocity_step: config.simulation.velocity_step_ms,
            angle_of_attack: config.simulation.angle_of_attack_deg,
            water_density: config.environment.water_density_kg_m3.unwrap_or(1025.0),
        };
        sim::run_sweep(&config.vehicle, &env, &sweep)?
    };

    // Aircraft: report stall speed (velocity sweep only)
    if !cli.aoa_sweep && config.simulation.vehicle_type == VehicleType::Aircraft {
        match results.iter().find(|r| r.airborne == Some(true)) {
            Some(r) => println!(
                "Stall speed: {:.1} m/s  ({:.1} km/h)",
                r.velocity_ms,
                r.velocity_ms * 3.6
            ),
            None => {
                println!("Warning: no airborne point found — increase velocity_end_ms in config")
            }
        }
    }

    // Determine output file stem
    std::fs::create_dir_all(&cli.out_dir)?;
    let stem = PathBuf::from(&config.simulation.output_file)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let csv_name = if cli.aoa_sweep {
        format!("{stem}_aoa_sweep.csv")
    } else {
        config.simulation.output_file.clone()
    };
    let csv_path = cli.out_dir.join(&csv_name);

    // Write CSV
    let mut writer = csv::Writer::from_path(&csv_path)?;
    for result in &results {
        writer.serialize(result)?;
    }
    writer.flush()?;
    println!("{} data points → {}", results.len(), csv_path.display());

    // Write JSON (opt-in)
    if cli.json {
        let json_path = csv_path.with_extension("json");
        let json = serde_json::to_string_pretty(&results)?;
        std::fs::write(&json_path, json)?;
        println!("JSON → {}", json_path.display());
    }

    Ok(())
}
