//! This module defines the `Boid` struct and its associated methods for representing and managing individual boids in the simulation.
//! Each `Boid` has a position and velocity, and methods for updating its state and drawing itself on the canvas.
//! The boids are represented as simple triangles that point in the direction of their velocity, and they wrap around the edges of the simulation window.

use crate::Vector2;
use rand::Rng;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{pixels::Color, render::WindowCanvas};

/// The `Boid` struct represents an individual boid in the simulation, with a position and velocity.
pub struct Boid {
    /// The current position of the boid in 2D space.
    pos: Vector2<f32>,
    /// The current velocity of the boid, which determines its movement direction and speed.
    vel: Vector2<f32>,
}

impl Boid {
    /// Creates a new `Boid` instance with a random velocity and the specified initial position.
    ///
    /// # Arguments
    /// * `x` - The initial x-coordinate of the boid's position.
    /// * `y` - The initial y-coordinate of the boid's position.
    ///
    /// # Returns
    /// A new `Boid` instance with the specified position and a random velocity.
    pub fn new(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();

        let vel_x = rng.gen_range(-2.0..2.0);
        let vel_y = rng.gen_range(-2.0..2.0);

        Boid {
            pos: Vector2 { x, y },
            vel: Vector2 { x: vel_x, y: vel_y },
        }
    }

    /// Updates the boid's position based on its velocity and wraps around the edges of the simulation window.
    ///
    /// # Arguments
    /// * `bound_width` - The width of the simulation window, used for wrapping around the x-coordinate.
    /// * `bound_height` - The height of the simulation window, used for wrapping around the y-coordinate.
    pub fn update(&mut self, bound_width: u32, bound_height: u32) {
        self.pos = self.pos + self.vel;

        if self.pos.x < 0.0 {
            self.pos.x = bound_width as f32
        }

        if self.pos.x > bound_width as f32 {
            self.pos.x = 0.0;
        }

        if self.pos.y < 0.0 {
            self.pos.y = bound_height as f32
        }

        if self.pos.y > bound_height as f32 {
            self.pos.y = 0.0;
        }
    }

    /// Draws the boid on the provided canvas as a triangle pointing in the direction of its velocity.
    /// If `show_directions` is true, it also draws a line indicating the boid's velocity direction.
    ///
    /// # Arguments
    /// * `canvas` - The canvas on which to draw the boid.
    /// * `show_directions` - A boolean flag indicating whether to draw the velocity direction line.
    ///
    /// # Returns
    /// A Result indicating success or failure of the drawing operation.
    pub fn draw(&self, canvas: &mut WindowCanvas, show_directions: bool) -> Result<(), String> {
        let half_height = 10.0; // The distance from the center of the boid to the tip of the triangle.
        let half_base = 4.0; // The distance from the center of the boid to the base corners of the triangle.

        // Calculate the angle of the velocity vector to determine the orientation of the triangle.
        let angle = self.vel.y.atan2(self.vel.x);

        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        // Define the vertices of the triangle relative to the boid's position, oriented along the velocity direction.
        let vertices = [
            Vector2 {
                x: half_height,
                y: 0.0,
            },
            Vector2 {
                x: -half_height,
                y: -half_base,
            },
            Vector2 {
                x: -half_height,
                y: half_base,
            },
        ];

        let mut screen_points_x = [0i16; 3];
        let mut screen_points_y = [0i16; 3];

        // Rotate and translate the triangle vertices to their final screen positions based on the boid's position and velocity angle.
        for (i, rel_p) in vertices.iter().enumerate() {
            let rotated_x = rel_p.x * cos_angle - rel_p.y * sin_angle;
            let rotated_y = rel_p.x * sin_angle + rel_p.y * cos_angle;

            let final_x = rotated_x + self.pos.x;
            let final_y = rotated_y + self.pos.y;

            screen_points_x[i] = final_x as i16;
            screen_points_y[i] = final_y as i16;
        }

        let color = Color::RGB(255, 0, 0);

        // Draw the filled triangle representing the boid, and then draw an anti-aliased outline for better visibility.
        canvas.filled_trigon(
            screen_points_x[0],
            screen_points_y[0],
            screen_points_x[1],
            screen_points_y[1],
            screen_points_x[2],
            screen_points_y[2],
            color,
        )?;

        canvas.aa_trigon(
            screen_points_x[0],
            screen_points_y[0],
            screen_points_x[1],
            screen_points_y[1],
            screen_points_x[2],
            screen_points_y[2],
            Color::RGB(255, 100, 100),
        )?;

        // If the `show_directions` flag is set, draw a line from the center of the boid in the direction of its velocity to indicate its movement direction.
        if show_directions {
            let debug_color = Color::RGB(100, 255, 255);
            let scale_factor = 30.0;

            let end_x = self.pos.x + (self.vel.x * scale_factor);
            let end_y = self.pos.y + (self.vel.y * scale_factor);

            canvas.line(
                self.pos.x as i16,
                self.pos.y as i16,
                end_x as i16,
                end_y as i16,
                debug_color,
            )?;
        }

        Ok(())
    }
}
