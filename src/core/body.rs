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
}
