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
    /// An optional index of the currently focused boid, if any. This allows the camera to follow a specific boid.
    focused_boid: Option<usize>,
    /// A flag to toggle the display of boid direction lines.
    show_directions: bool,
    /// The x-coordinate of the camera's center position in the world.
    cam_x: f32,
    /// The y-coordinate of the camera's center position in the world.
    cam_y: f32,
    /// The zoom level of the camera, allowing the user to zoom in and out of the simulation.
    zoom: f32,
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
        let mut canvas = window
            .clone()
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

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
            focused_boid: None,
            show_directions: false,
            cam_x: width as f32 / 2.0,
            cam_y: height as f32 / 2.0,
            zoom: 1.0,
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

                    Event::KeyDown {
                        keycode: Some(Keycode::R),
                        ..
                    } => {
                        self.focused_boid = None;
                        self.zoom = 1.0;
                    }

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

                    // Handle mouse wheel events to zoom in and out of the simulation.
                    Event::MouseWheel { y, .. } => {
                        if y > 0 {
                            self.zoom *= 1.1;
                        } else if y < 0 {
                            self.zoom *= 0.9;
                        }

                        self.zoom = self.zoom.clamp(1.0, 5.0);
                    }

                    // Handle mouse button down events to select a boid and focus the camera on it.
                    Event::MouseButtonDown { x, y, .. } => {
                        // Convert the mouse coordinates to world coordinates based on the current camera position and zoom level.
                        let world_x = (x as f32 - width as f32 / 2.0) / self.zoom + self.cam_x;
                        let world_y = (y as f32 - height as f32 / 2.0) / self.zoom + self.cam_y;

                        self.focused_boid = None;

                        // Iterate through the boids to find if any are within a certain distance of the mouse click position, and if so, set that boid as the focused boid.
                        for (idx, boid) in self.boids.iter().enumerate() {
                            let dx = world_x - boid.pos.x;
                            let dy = world_y - boid.pos.y;
                            let distance = (dx * dx + dy * dy).sqrt();

                            if distance < (25.0 / self.zoom) {
                                self.focused_boid = Some(idx);
                                break;
                            }
                        }
                    }

                    _ => {}
                }
            }

            // Update the state of each boid based on the simulation rules and the current window dimensions.
            for boid in self.boids.iter_mut() {
                boid.update(width, height);
            }

            // Update the camera position to follow the focused boid if there is one, otherwise center the camera on the middle of the window.
            if let Some(idx) = self.focused_boid {
                if let Some(boid) = self.boids.get(idx) {
                    self.cam_x = boid.pos.x;
                    self.cam_y = boid.pos.y;
                }
            } else {
                self.cam_x = width as f32 / 2.0;
                self.cam_y = height as f32 / 2.0;
            }

            self.canvas.set_draw_color(Color::RGB(10, 10, 20));
            self.canvas.clear();

            // Draw each boid on the canvas, optionally showing their direction lines based on the current setting.
            for (idx, boid) in self.boids.iter().enumerate() {
                let is_selected = self.focused_boid == Some(idx);
                boid.draw(
                    &mut self.canvas,
                    self.show_directions,
                    is_selected,
                    self.cam_x,
                    self.cam_y,
                    self.zoom,
                    width as f32,
                    height as f32,
                )?;
            }

            // Draw UI text on the canvas to provide instructions and information about the simulation.
            self.draw_sim_commands()?;

            self.canvas.present();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}

impl Simulation {
    /// Draws the UI text on the canvas, providing instructions for controlling the simulation and displaying the current number of boids.
    ///
    /// # Returns
    /// A Result indicating success or failure of the drawing operation.
    fn draw_sim_commands(&mut self) -> Result<(), String> {
        let text_color = Color::RGB(255, 255, 255);
        let accent_color = Color::RGB(100, 255, 100);

        self.canvas.string(10, 10, "BOID SIMULATION", text_color)?;
        self.canvas
            .string(10, 25, "UP Arrow   : Add Boid", text_color)?;
        self.canvas
            .string(10, 40, "DOWN arrow : Remove Boid", text_color)?;
        self.canvas
            .string(10, 55, "D          : Show Directions", text_color)?;

        self.canvas
            .string(10, 70, "Left Click : Select & Follow Boid", text_color)?;
        self.canvas
            .string(10, 85, "Mouse Wheel: Zoom In/Out", text_color)?;
        self.canvas
            .string(10, 100, "R          : Reset View & Zoom", text_color)?;

        self.canvas
            .string(10, 115, "ESC        : Exit", text_color)?;

        let info_text = format!("Total Boids: {}", self.boids.len());
        self.canvas.string(10, 135, &info_text, accent_color)?;

        Ok(())
    }
}
