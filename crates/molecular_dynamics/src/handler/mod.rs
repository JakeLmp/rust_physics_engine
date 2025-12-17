use itertools::Itertools;
use physics_core::vector::Vector2D;
use uom::si::{
    f64::{Acceleration, Length, Mass, Ratio, Time, Velocity},
    ratio::ratio,
};
use visualization::simulation::config::{BoundaryKind, ParallelComputationKind, SimulationConfig};

use crate::{
    physics::{potential::Potential, time_integration::StepType},
    point_mass::PointMass,
};

pub struct SimulationHandler {
    pub points: Vec<PointMass>,

    positions: Vec<Vector2D<Length>>,
    velocities: Vec<Vector2D<Velocity>>,
    accelerations: Vec<Vector2D<Acceleration>>,
    masses: Vec<Mass>,

    last_positions: Vec<Vector2D<Length>>,
    last_velocities: Vec<Vector2D<Velocity>>,
}

impl SimulationHandler {
    pub fn new(points: Vec<PointMass>) -> Self {
        let len = points.len();
        let mut handler = Self {
            points,
            positions: vec![Vector2D::<Length>::zero(); len],
            velocities: vec![Vector2D::<Velocity>::zero(); len],
            accelerations: vec![Vector2D::<Acceleration>::zero(); len],
            masses: vec![Mass::default(); len],
            last_positions: vec![Vector2D::<Length>::zero(); len],
            last_velocities: vec![Vector2D::<Velocity>::zero(); len],
        };

        handler.sync_from_points();
        handler
    }

    pub fn sync_from_points(&mut self) {
        for (i, point) in self.points.iter().enumerate() {
            self.positions[i] = point.pos();
            self.velocities[i] = point.vel();
            self.accelerations[i] = point.acc();
            self.masses[i] = point.mass();
        }
    }

    pub fn sync_to_points(&mut self) {
        for (i, point) in self.points.iter_mut().enumerate() {
            point.set_pos(self.positions[i]);
            point.set_vel(self.velocities[i]);
            point.set_acc(self.accelerations[i]);
            point.set_mass(self.masses[i]);
        }
    }

    pub fn step_physics(
        &mut self,
        config: &SimulationConfig,
        potential: &impl Potential,
        time_step: Time,
        movement_step_type: StepType,
    ) {
        for _ in 0..config.time_steps_per_frame {
            // Calculate forces and accelerations at current positions
            self.accelerations.fill(Vector2D::<Acceleration>::zero());

            match config.parallel_computation_kind {
                ParallelComputationKind::SingleThread => {
                    for (i, j) in (0..self.accelerations.len()).tuple_combinations() {
                        let force = potential.force_from_arrays(
                            i,
                            j,
                            &self.positions,
                            &self.velocities,
                            &self.accelerations,
                            &self.masses,
                            config,
                        );
                        self.accelerations[i] += force / self.masses[i];
                        self.accelerations[j] -= force / self.masses[j];
                    }
                }
                ParallelComputationKind::CPUMultiThread => {
                    todo!();
                }
            }

            // Update positions and velocities
            for i in 0..self.positions.len() {
                self.step_movement(i, time_step, &movement_step_type);

                // Apply Periodic and Elastic boundary conditions
                match &config.boundary_type {
                    BoundaryKind::Periodic(bounds) => self.apply_periodic_boundary(i, bounds),
                    BoundaryKind::Elastic(bounds) => self.apply_elastic_boundary(i, bounds),
                    _ => {}
                }
            }

            // For Velocity Verlet: recalculate accelerations at NEW positions
            // to complete the second half-step
            if matches!(movement_step_type, StepType::VelocityVerlet) {
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
                            config,
                        );
                        self.accelerations[i] += force / self.masses[i];
                        self.accelerations[j] -= force / self.masses[j];
                    }
                }

                // Complete the second velocity half-step
                for i in 0..self.velocities.len() {
                    self.velocities[i] +=
                        (time_step / Ratio::new::<ratio>(2.0)) * self.accelerations[i];
                }
            }

            // If open boundaries, remove outside
            if let BoundaryKind::Periodic(bounds) = config.boundary_type {
                self.apply_open_boundary(&bounds);
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

        self.velocities[idx] += (time_step / Ratio::new::<ratio>(2.0)) * self.accelerations[idx];
        self.positions[idx] += time_step * self.velocities[idx];

        // Note: second velocity half-step done in step_physics after recalculating forces

        self.last_positions[idx] = current_pos;
        self.last_velocities[idx] = current_vel;
    }

    fn apply_periodic_boundary(&mut self, idx: usize, bounds: &Vector2D<Length>) {
        self.positions[idx].x = Self::wrap_coordinate(self.positions[idx].x, bounds.x);
        self.positions[idx].y = Self::wrap_coordinate(self.positions[idx].y, bounds.y);
    }

    fn wrap_coordinate(pos: Length, bound: Length) -> Length {
        let double_bound = bound * Ratio::new::<ratio>(2.0);
        let shifted = pos + bound;

        let r = shifted / double_bound;
        let floored = r.value.floor();
        let wrapped = shifted - Ratio::new::<ratio>(floored) * double_bound;

        wrapped - bound
    }

    fn apply_elastic_boundary(&mut self, idx: usize, bounds: &Vector2D<Length>) {
        if self.positions[idx].x > bounds.x {
            self.positions[idx].x = bounds.x;
            self.velocities[idx].x = -self.velocities[idx].x;
        } else if self.positions[idx].x < -bounds.x {
            self.positions[idx].x = -bounds.x;
            self.velocities[idx].x = -self.velocities[idx].x;
        }

        if self.positions[idx].y > bounds.y {
            self.positions[idx].y = bounds.y;
            self.velocities[idx].y = -self.velocities[idx].y;
        } else if self.positions[idx].y < -bounds.y {
            self.positions[idx].y = -bounds.y;
            self.velocities[idx].y = -self.velocities[idx].y;
        }
    }

    fn apply_open_boundary(&mut self, bounds: &Vector2D<Length>) {
        let mut i = 0;
        while i < self.positions.len() {
            if self.is_outside_boundary(i, bounds) {
                self.positions.swap_remove(i);
                self.velocities.swap_remove(i);
                self.accelerations.swap_remove(i);
                self.masses.swap_remove(i);
                self.last_positions.swap_remove(i);
                self.last_velocities.swap_remove(i);
                self.points.swap_remove(i);
                // Don't increment i, check the swapped element
            } else {
                i += 1;
            }
        }
    }

    fn is_outside_boundary(&self, idx: usize, bounds: &Vector2D<Length>) -> bool {
        self.positions[idx].x.abs() > bounds.x || self.positions[idx].y.abs() > bounds.y
    }
}
