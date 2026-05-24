/// Properties of the fluid (air) the body moves through.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FluidEnvironment {
    /// Air density [kg/m³].
    pub air_density: f64,
    /// Ambient temperature [°C].
    pub temperature: f64,
    /// Headwind speed (positive = into the vehicle) [m/s].
    pub wind_speed: f64,
}

impl FluidEnvironment {
    /// ISA sea-level standard atmosphere, no wind.
    #[allow(dead_code)]
    pub fn standard_air() -> Self {
        Self {
            air_density: 1.225,
            temperature: 15.0,
            wind_speed: 0.0,
        }
    }
}
