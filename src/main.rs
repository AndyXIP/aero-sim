mod aero;
mod core;
mod error;
mod input;
mod sim;

use std::{env, path::PathBuf};

use error::AeroError;

fn main() -> Result<(), AeroError> {
    let args: Vec<String> = env::args().collect();
    let config_path = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("examples/f1_car.toml"));

    println!("Loading config: {}", config_path.display());
    let config = input::load_config(&config_path)?;
    println!("Vehicle: {}", config.vehicle.name);

    let env = core::FluidEnvironment {
        air_density: config.environment.air_density_kg_m3,
        temperature: config.environment.temperature_c,
        wind_speed: config.environment.wind_speed_ms,
    };

    let sweep = sim::SweepConfig {
        velocity_start: config.simulation.velocity_start_ms,
        velocity_end: config.simulation.velocity_end_ms,
        velocity_step: config.simulation.velocity_step_ms,
        angle_of_attack: config.simulation.angle_of_attack_deg,
    };

    let results = sim::run_sweep(&config.vehicle, &env, &sweep)?;

    std::fs::create_dir_all("output")?;
    let out_path = format!("output/{}", config.simulation.output_file);
    let mut writer = csv::Writer::from_path(&out_path)?;
    for result in &results {
        writer.serialize(result)?;
    }
    writer.flush()?;

    println!("{} data points written to {}", results.len(), out_path);

    Ok(())
}
