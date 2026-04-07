# Boids Simulation in Rust

## Overview

This project is a 2D artificial life program that simulates the flocking behavior of birds, written in Rust. It serves as a foundational engine to implement the "Boids" algorithm, originally developed by Craig Reynolds in 1986.

The simulation utilizes the SDL2 library (and SDL2_gfx) for rendering the environment, providing smooth, anti-aliased graphics and real-time performance. Currently, the project features the core rendering engine, movement logic with screen wrap-around, and real-time user inputs.

## The Boids Algorithm

The Boids algorithm creates complex, emergent flocking behavior through three simple steering behaviors applied to individual entities (boids):

1. **Separation:** Steering to avoid crowding local flockmates.
2. **Alignment:** Steering towards the average heading of local flockmates.
3. **Cohesion:** Steering to move towards the average position (center of mass) of local flockmates.

_(Note: The physics rules are currently in the implementation phase)._

## Prerequisites

To compile and run this project, you need to have Rust and Cargo installed, along with the SDL2 system libraries.

### Linux Dependencies

Depending on your distribution, you will need to install the SDL2 development packages. For Debian/Ubuntu-based systems:

```bash
sudo apt update
sudo apt install libsdl2-dev libsdl2-gfx-dev
```

## Running the Project

Clone the repository and run the project using Cargo:

```bash
cargo run
```

The first build might take a few moments as Cargo downloads and compiles the required crates, including the Rust bindings for SDL2 and the random number generation library (`rand`).

## Controls

You can interact with the simulation in real-time using the following keyboard controls:

- **Up Arrow** : Add a new boid to the simulation at a random position.
- **Down Arrow** : Remove a boid from the simulation.
- **D** : Toggle debug mode. This displays a directional vector for each boid, representing its current velocity and heading.
- **Left Click** : Select a boid to focus on.
- **Mouse Wheel** : Zoom in and out of the simulation view.
- **R** : Reset the simulation by clearing all existing boids and starting fresh.
- **ESC** : Close the window and exit the simulation.

## Project Structure

- `main.rs`: Entry point of the application.
- `simulation.rs`: Contains the main game loop, event handling, and rendering calls.
- `boid.rs`: Defines the Boid entity, its movement updates, and graphical representation (triangle math and rotation).
- `vector.rs`: A custom generic 2D mathematical vector structure with operator overloading for physics calculations.
