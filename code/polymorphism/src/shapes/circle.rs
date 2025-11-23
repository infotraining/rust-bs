use crate::shapes::traits::{Drawable, Shape};
use crate::shapes::{point::Point};

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    pub position: Point,
    pub radius: u32
}

impl Shape for Circle {
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.position.x += dx;
        self.position.y += dy;
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius as f64) * (self.radius as f64)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        println!(
            "Drawing Circle at ({}, {}) with radius {}",
            self.position.x, self.position.y, self.radius
        );
    }
}

