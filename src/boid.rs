//! This module defines the `Boid` struct and its associated methods for representing and managing individual boids in the simulation.
//! Each `Boid` has a position and velocity, and methods for updating its state and drawing itself on the canvas.
//! The boids are represented as simple triangles that point in the direction of their velocity, and they wrap around the edges of the simulation window.

use crate::Vector2;
use rand::Rng;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{pixels::Color, render::WindowCanvas};

/// The `Boid` struct represents an individual boid in the simulation, with a position and velocity.
#[derive(Debug, Clone, Copy)]
pub struct Boid {
    /// The current position of the boid in 2D space.
    pub pos: Vector2<f32>,
    /// The current velocity of the boid, which determines its movement direction and speed.
    pub vel: Vector2<f32>,
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
    pub fn draw(
        &self,
        canvas: &mut WindowCanvas,
        show_directions: bool,
        is_selected: bool,
        cam_x: f32,
        cam_y: f32,
        zoom: f32,
        screen_w: f32,
        screen_h: f32,
    ) -> Result<(), String> {
        let (screen_points_x, screen_points_y) =
            self.calculate_triangle_screen_points(zoom, cam_x, cam_y, screen_w, screen_h);

        if is_selected {
            self.draw_selected_triangle(canvas, &screen_points_x, &screen_points_y)?;
        } else {
            self.draw_not_selected_triangle(canvas, &screen_points_x, &screen_points_y)?;
        }

        // If the `show_directions` flag is set, draw a line from the center of the boid in the direction of its velocity to indicate its movement direction.
        if show_directions {
            self.draw_direction_line(canvas, zoom, cam_x, cam_y, screen_w, screen_h)?;
        }

        Ok(())
    }
}

impl Boid {
    /// Calculates the screen coordinates of the vertices of the triangle representing the boid, based on its position, velocity, and the current camera settings.
    /// The triangle is oriented in the direction of the boid's velocity, and its size is scaled by the zoom level.
    ///
    /// # Arguments
    /// * `zoom` - The current zoom level of the camera, which scales the size of the triangle.
    /// * `cam_x` - The x-coordinate of the camera's center position in the world, used to calculate the boid's position relative to the camera.
    /// * `cam_y` - The y-coordinate of the camera's center position in the world, used to calculate the boid's position relative to the camera.
    /// * `screen_w` - The width of the screen in pixels, used to calculate the final screen coordinates of the triangle vertices.
    /// * `screen_h` - The height of the screen in pixels, used to calculate the final screen coordinates of the triangle vertices.
    ///
    /// # Returns
    /// A tuple containing two arrays: the first array contains the x-coordinates of the triangle vertices, and the second array contains the y-coordinates of the triangle vertices, both as i16
    /// to be used for drawing the triangle on the screen.
    fn calculate_triangle_screen_points(
        &self,
        zoom: f32,
        cam_x: f32,
        cam_y: f32,
        screen_w: f32,
        screen_h: f32,
    ) -> ([i16; 3], [i16; 3]) {
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

            let scaled_x = rotated_x * zoom;
            let scaled_y = rotated_y * zoom;

            let final_x = (self.pos.x - cam_x) * zoom + (screen_w / 2.0) + scaled_x;
            let final_y = (self.pos.y - cam_y) * zoom + (screen_h / 2.0) + scaled_y;

            screen_points_x[i] = final_x as i16;
            screen_points_y[i] = final_y as i16;
        }

        (screen_points_x, screen_points_y)
    }

    /// Draws a line from the center of the boid in the direction of its velocity to indicate its movement direction, using a specified color and scaling factor.
    /// The line is drawn on the provided canvas, and its length is determined by the boid's velocity and a fixed scale factor to make it visually distinguishable.
    /// The line's start and end points are calculated based on the boid's position, velocity, and the current camera settings to ensure it is correctly positioned on the screen.
    ///
    /// # Arguments
    /// * `canvas` - The canvas on which to draw the direction line.
    /// * `zoom` - The current zoom level of the camera, which scales the length of the direction line.
    /// * `cam_x` - The x-coordinate of the camera's center position in the world, used to calculate the line's position relative to the camera.
    /// * `cam_y` - The y-coordinate of the camera's center position in the world, used to calculate the line's position relative to the camera.
    /// * `screen_w` - The width of the screen in pixels, used to calculate the final screen coordinates of the line's start and end points.
    /// * `screen_h` - The height of the screen in pixels, used to calculate the final screen coordinates of the line's start and end points.
    ///
    /// # Returns
    /// A Result indicating success or failure of the drawing operation.
    fn draw_direction_line(
        &self,
        canvas: &mut WindowCanvas,
        zoom: f32,
        cam_x: f32,
        cam_y: f32,
        screen_w: f32,
        screen_h: f32,
    ) -> Result<(), String> {
        let debug_color = Color::RGB(100, 255, 255);

        // A fixed scale factor to determine the length of the direction line based on the boid's velocity, making it visually distinguishable on the screen.
        let scale_factor = 30.0;

        // Calculate the end point of the direction line based on the boid's velocity and a fixed scale factor to make it visually distinguishable.
        let end_x = self.pos.x + (self.vel.x * scale_factor);
        let end_y = self.pos.y + (self.vel.y * scale_factor);

        // Calculate the screen coordinates of the start and end points of the direction line based on the boid's position, velocity, and the current camera settings to ensure it is correctly positioned on the screen.
        let start_px = (self.pos.x - cam_x) * zoom + (screen_w / 2.0);
        let start_py = (self.pos.y - cam_y) * zoom + (screen_h / 2.0);
        let end_px = (end_x - cam_x) * zoom + (screen_w / 2.0);
        let end_py = (end_y - cam_y) * zoom + (screen_h / 2.0);

        canvas.set_draw_color(debug_color);
        canvas.draw_line(
            sdl2::rect::Point::new(start_px as i32, start_py as i32),
            sdl2::rect::Point::new(end_px as i32, end_py as i32),
        )?;

        Ok(())
    }

    /// Draws the triangle representing the boid on the canvas when it is not selected, using a filled triangle with a specific color and an anti-aliased outline for better visibility.
    /// The triangle is drawn using the provided screen coordinates of its vertices, which are calculated based on the boid's position, velocity, and the current camera settings to ensure it is correctly positioned on the screen.
    /// The filled triangle is drawn with a solid color, while the anti-aliased outline is drawn with a lighter color to create a visual distinction and make the boid more visible against the background.
    /// The method returns a Result indicating success or failure of the drawing operation, allowing for error handling in case of issues with the canvas or drawing operations.
    ///
    ///  # Arguments
    /// * `canvas` - The canvas on which to draw the triangle representing the boid.
    /// * `screen_points_x` - An array containing the x-coordinates of the triangle vertices, calculated based on the boid's position, velocity, and camera settings.
    /// * `screen_points_y` - An array containing the y-coordinates of the triangle vertices, calculated based on the boid's position, velocity, and camera settings.
    ///
    /// # Returns
    /// A Result indicating success or failure of the drawing operation.
    fn draw_not_selected_triangle(
        &self,
        canvas: &mut WindowCanvas,
        screen_points_x: &[i16],
        screen_points_y: &[i16],
    ) -> Result<(), String> {
        canvas.filled_trigon(
            screen_points_x[0],
            screen_points_y[0],
            screen_points_x[1],
            screen_points_y[1],
            screen_points_x[2],
            screen_points_y[2],
            Color::RGB(200, 0, 0),
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

        Ok(())
    }

    /// Draws the triangle representing the boid on the canvas when it is selected, using a specific color and additional lines to indicate selection.
    /// The triangle is drawn using the provided screen coordinates of its vertices, which are calculated based on the boid's position, velocity, and the current camera settings to ensure it is correctly positioned on the screen.
    /// When the boid is selected, it is drawn with a bright green color, and additional lines are drawn from the tip of the triangle to points along the base to create a visual indication of selection, making it stand out from non-selected boids.
    /// The method returns a Result indicating success or failure of the drawing operation, allowing for error handling in case of issues with the canvas or drawing operations.
    ///
    /// # Arguments
    /// * `canvas` - The canvas on which to draw the triangle representing the boid.
    /// * `screen_points_x` - An array containing the x-coordinates of the triangle vertices, calculated based on the boid's position, velocity, and camera settings.
    /// * `screen_points_y` - An array containing the y-coordinates of the triangle vertices, calculated based on the boid's position, velocity, and camera settings.
    ///
    /// # Returns
    /// A Result indicating success or failure of the drawing operation.
    fn draw_selected_triangle(
        &self,
        canvas: &mut WindowCanvas,
        screen_points_x: &[i16],
        screen_points_y: &[i16],
    ) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 255, 0));

        // Draw the filled triangle with a bright green color to indicate selection.
        let p1 = sdl2::rect::Point::new(screen_points_x[0] as i32, screen_points_y[0] as i32);
        let p2 = sdl2::rect::Point::new(screen_points_x[1] as i32, screen_points_y[1] as i32);
        let p3 = sdl2::rect::Point::new(screen_points_x[2] as i32, screen_points_y[2] as i32);

        canvas.draw_line(p1, p2)?;
        canvas.draw_line(p2, p3)?;
        canvas.draw_line(p3, p1)?;

        // Draw additional lines from the tip of the triangle to points along the base to create a visual indication of selection.
        let steps = 15;
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let base_x = p2.x as f32 + (p3.x as f32 - p2.x as f32) * t;
            let base_y = p2.y as f32 + (p3.y as f32 - p2.y as f32) * t;

            canvas.draw_line(p1, sdl2::rect::Point::new(base_x as i32, base_y as i32))?;
        }

        Ok(())
    }
}
