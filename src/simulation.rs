//! The simulation module contains the main loop and rendering logic for the boids simulation.
//! It handles user input, updates the state of the boids, and renders them on the screen using SDL2.

use rand::Rng;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{
    EventPump, event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window,
};
use std::time::Duration;

use crate::Boid;

/// The Simulation struct encapsulates the SDL2 window, canvas, event pump, and the list of boids.
pub struct Simulation {
    /// The SDL2 window where the simulation is rendered.
    window: Window,
    /// The canvas used for drawing the boids and UI elements.
    canvas: Canvas<Window>,
    /// The event pump for handling user input and events.
    event_pump: EventPump,
    /// A vector containing all the boids in the simulation.
    boids: Vec<Boid>,
    /// A flag to toggle the display of boid direction lines.
    show_directions: bool,
}

impl Simulation {
    /// Creates a new Simulation instance with the specified title, width, and height.
    ///
    /// # Arguments
    /// * `title` - The title of the simulation window.
    /// * `width` - The width of the simulation window in pixels.
    /// * `height` - The height of the simulation window in pixels.
    ///
    /// # Returns
    /// A Result containing the Simulation instance if successful, or a String error message if initialization fails.
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
        let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

        // Create the SDL2 window and canvas for rendering the simulation.
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        // Create the canvas for drawing.
        let canvas = window
            .clone()
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        // Initialize the event pump for handling user input and events.
        let event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

        // Initialize a vector of boids with random positions within the window dimensions.
        let num_boids = 50;
        let mut boids = Vec::with_capacity(num_boids);
        let mut rng = rand::thread_rng();

        for _ in 0..num_boids {
            boids.push(Boid::new(
                rng.gen_range(0..width) as f32,
                rng.gen_range(0..height) as f32,
            ));
        }

        Ok(Simulation {
            window,
            canvas,
            event_pump,
            boids,
            show_directions: false,
        })
    }

    /// Runs the main loop of the simulation, handling events, updating boid states, and rendering.
    ///
    /// # Notes
    /// - The loop continues until the user quits the simulation (e.g., by pressing the Escape key or closing the window).
    ///
    /// # Returns
    /// A Result indicating success or failure of the simulation loop.
    pub fn run(mut self) -> Result<(), String> {
        let (width, height) = self.window.size();

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    // Handle quit events and key presses for controlling the simulation.
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,

                    // Add a new boid at a random position when the UP arrow key is pressed.
                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        ..
                    } => {
                        let mut rng = rand::thread_rng();
                        self.boids.push(Boid::new(
                            rng.gen_range(0..width) as f32,
                            rng.gen_range(0..height) as f32,
                        ));
                    }

                    // Remove the last boid from the simulation when the DOWN arrow key is pressed.
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        ..
                    } => {
                        self.boids.pop();
                    }

                    // Toggle the display of boid direction lines when the 'D' key is pressed.
                    Event::KeyDown {
                        keycode: Some(Keycode::D),
                        ..
                    } => {
                        self.show_directions = !self.show_directions;
                    }

                    _ => {}
                }
            }

            // Update the state of each boid based on the simulation rules and the current window dimensions.
            for boid in self.boids.iter_mut() {
                boid.update(width, height);
            }

            self.canvas.set_draw_color(Color::RGB(10, 10, 20));
            self.canvas.clear();

            // Draw each boid on the canvas, optionally showing their direction lines based on the current setting.
            for boid in self.boids.iter() {
                boid.draw(&mut self.canvas, self.show_directions)?;
            }

            // Draw UI text on the canvas to provide instructions and information about the simulation.
            let text_color = Color::RGB(255, 255, 255);
            let accent_color = Color::RGB(100, 255, 100);

            self.canvas.string(10, 10, "BOID SIMULATION", text_color)?;
            self.canvas
                .string(10, 25, "UP Arrow   : Add Boid", text_color)?;
            self.canvas
                .string(10, 40, "DOWN arrow : Remove Boid", text_color)?;
            self.canvas
                .string(10, 55, "D          : Show Boids Direction", text_color)?;
            self.canvas
                .string(10, 70, "ESC        : Exit", text_color)?;

            let info_text = format!("Total Boids: {}", self.boids.len());
            self.canvas.string(10, 90, &info_text, accent_color)?;

            self.canvas.present();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}
