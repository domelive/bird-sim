//! The main entry point for the boids simulation. This file initializes the simulation and starts the main loop.

use bird_sim::Simulation;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 900;
const TITLE: &str = "Boid Simulation";

fn main() -> Result<(), String> {
    let sim = Simulation::new(TITLE, WIDTH, HEIGHT)?;
    sim.run()
}
