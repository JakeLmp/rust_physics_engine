use physics_core::vector::Vector2D;
use uom::si::{
    f64::{Acceleration, Length, Mass, Ratio, Time, Velocity},
    ratio::ratio,
};
use visualization::simulation::config::SimulationConfig;

use crate::{
    objects::physical_object::PhysicalObject,
    physics::{potential::Potential, time_integration::StepType},
};

pub struct SimulationHandler {
    pub objects: Vec<Box<dyn PhysicalObject>>,

    positions: Vec<Vector2D<Length>>,
    velocities: Vec<Vector2D<Velocity>>,
    accelerations: Vec<Vector2D<Acceleration>>,
    masses: Vec<Mass>,

    last_positions: Vec<Vector2D<Length>>,
    last_velocities: Vec<Vector2D<Velocity>>,
}

impl SimulationHandler {
    pub fn new(objects: Vec<Box<dyn PhysicalObject>>) -> Self {
        let len = objects.len();
        let mut handler = Self {
            objects,
            positions: vec![Vector2D::<Length>::zero(); len],
            velocities: vec![Vector2D::<Velocity>::zero(); len],
            accelerations: vec![Vector2D::<Acceleration>::zero(); len],
            masses: vec![Mass::default(); len],
            last_positions: vec![Vector2D::<Length>::zero(); len],
            last_velocities: vec![Vector2D::<Velocity>::zero(); len],
        };

        handler.sync_from_objects();
        handler
    }

    pub fn sync_from_objects(&mut self) {
        for (i, obj) in self.objects.iter().enumerate() {
            self.positions[i] = obj.pos();
            self.velocities[i] = obj.vel();
            self.accelerations[i] = obj.acc();
            self.masses[i] = obj.mass();
        }
    }

    pub fn sync_to_objects(&mut self) {
        for (i, obj) in self.objects.iter_mut().enumerate() {
            obj.set_pos(self.positions[i]);
            obj.set_vel(self.velocities[i]);
            obj.set_acc(self.accelerations[i]);
            obj.set_mass(self.masses[i]);
        }
    }

    pub fn step_physics(
        &mut self,
        config: &SimulationConfig,
        potential: &impl Potential,
        time_step: Time,
        movement_step_type: StepType,
    ) {
        for _ in 0..config.time_steps_per_frame.unwrap_or(1) {
            for acc in &mut self.accelerations {
                *acc = Vector2D::<Acceleration>::zero();
            }

            for i in 0..self.accelerations.len() {
                for j in (i + 1)..self.accelerations.len() {
                    let force = potential.force_from_arrays(
                        i,
                        j,
                        &self.positions,
                        &self.velocities,
                        &self.accelerations,
                        &self.masses,
                    );
                    self.accelerations[i] += force / self.masses[i];
                    self.accelerations[j] -= force / self.masses[j];
                }
            }

            for i in 0..self.positions.len() {
                self.step_movement(i, time_step, &movement_step_type)
            }
        }
    }

    fn step_movement(&mut self, idx: usize, time_step: Time, step_type: &StepType) {
        match step_type {
            StepType::Naive => self.naive_step(idx, time_step),
            StepType::Verlet => self.verlet_step(idx, time_step),
            StepType::VelocityVerlet => self.velocity_verlet_step(idx, time_step),
        }
    }

    fn naive_step(&mut self, idx: usize, time_step: Time) {
        self.positions[idx] += time_step * self.velocities[idx];
        self.velocities[idx] += time_step * self.accelerations[idx];
    }

    fn verlet_step(&mut self, idx: usize, time_step: Time) {
        let prev_pos = self.last_positions[idx];
        let current_pos = self.positions[idx];

        self.positions[idx] = Ratio::new::<ratio>(2.) * self.positions[idx]
            - self.last_positions[idx]
            + time_step * (time_step * self.accelerations[idx]);

        self.velocities[idx] =
            (self.positions[idx] - prev_pos) / (Ratio::new::<ratio>(2.) * time_step);

        self.last_positions[idx] = current_pos;
    }

    fn velocity_verlet_step(&mut self, idx: usize, time_step: Time) {
        let current_pos = self.positions[idx];
        let current_vel = self.velocities[idx];

        self.positions[idx] += time_step * self.velocities[idx]
            + (time_step / Ratio::new::<ratio>(2.0)) * (time_step * self.accelerations[idx]);

        self.velocities[idx] += (time_step / Ratio::new::<ratio>(2.0))
            * (Ratio::new::<ratio>(2.0) * self.accelerations[idx]);

        self.last_positions[idx] = current_pos;
        self.last_velocities[idx] = current_vel;
    }
}
