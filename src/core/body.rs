/// Trait implemented by any vehicle or shape that can be simulated aerodynamically.
pub trait AerodynamicBody {
    /// Dimensionless drag coefficient (Cd).
    fn drag_coefficient(&self) -> f64;

    /// Dimensionless lift coefficient (Cl). Negative values indicate downforce.
    fn lift_coefficient(&self) -> f64;

    /// Frontal (projected) area used in drag calculations [m²].
    fn frontal_area(&self) -> f64;

    /// Reference (planform / wing) area used in lift calculations [m²].
    fn reference_area(&self) -> f64;

    /// Total mass of the body [kg].
    fn mass(&self) -> f64;

    /// Critical AoA at which stall begins [degrees]. Aircraft only. Default: 16°.
    fn stall_aoa_deg(&self) -> f64 {
        16.0
    }

    /// Hull drag coefficient (Cd_hull). Marine vessels only. `None` for others.
    fn hull_drag_coefficient(&self) -> Option<f64> {
        None
    }

    /// Wetted hull area [m²]. Marine vessels only. `None` for others.
    fn wetted_area_m2(&self) -> Option<f64> {
        None
    }
}
