use uom::si::f64::Time;

#[allow(dead_code)]
pub enum StepType {
    /// Naive update method (Rₖ₊₁ = Rₖ + τ × Vₖ) and equiv. for velocity
    Naive,
    /// Base Verlet update method
    Verlet,
    /// Velocity version of Verlet update method
    VelocityVerlet,
}

pub trait NaiveStep {
    fn naive_step(&mut self, time_step: Time);
}

pub trait VerletStep {
    fn verlet_step(&mut self, time_step: Time);
}

pub trait VelocityVerletStep {
    fn velocity_verlet_step(&mut self, time_step: Time);
}
