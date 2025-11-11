mod objects;
mod physics;
mod simulation;

use uom::si::{
    f64::{Length, Time},
    length::meter,
    time::second,
};

use macroquad::prelude::*;

use physics::{
    cluster::{Cluster, RectangularBounds},
    potential::{Gravity, Potential},
};
use simulation::config::*;
use simulation::screen::Screen;
use simulation::units::*;

use crate::physics::time_integration::StepType;

#[macroquad::main("Physics Engine")]
async fn main() {
    let mut passed_time = Time::new::<second>(0.0);

    // Simulation config for cluster demo
    let config = SimulationConfig {
        time_step: Time::new::<second>(100.),
        length_unit: LengthUnit::Meter,
        mass_unit: MassUnit::Kilogram,
        pixels_per_length: 2.0,
        display_stats: true,
    };

    // Define bounds for cluster initialization
    let bounds = RectangularBounds {
        x1: Length::new::<meter>(-200.0),
        x2: Length::new::<meter>(200.0),
        y1: Length::new::<meter>(-100.0),
        y2: Length::new::<meter>(100.0),
    };

    // Create cluster
    let mut cluster = Cluster::new(&config, &bounds);

    // Newtonian gravity potential
    let potential = Gravity::default();

    loop {
        clear_background(BLACK);

        // Step the cluster simulation
        cluster.step(&config, &potential, Some(&StepType::Verlet));

        // Draw all objects in the cluster
        for object in &cluster.objects {
            object.draw(&config, None, WHITE);
        }

        // Draw center of mass
        let com = cluster.center_of_mass();
        let screen_pos = Screen::world_to_screen(&com, &config);
        draw_circle(screen_pos.x, screen_pos.y, 5.0, RED);

        // Update and display total passed time
        passed_time += config.time_step;

        if config.display_stats {
            Screen::display_stats(
                &[("time", &(passed_time.value as f32))],
                simulation::screen::ScreenPosition::TopRight,
                None,
                None,
                None,
                None,
            );
        }

        next_frame().await;
    }
}
