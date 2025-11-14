use crate::shapes::traits::{Shape, Drawable};
use crate::shapes::point::Point;

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub position: Point,
    pub width: u32,
    pub height: u32
}

impl Shape for Rectangle {

    fn move_by(&mut self, dx: i32, dy: i32) {
        self.position.x += dx;
        self.position.y += dy;
    }

    fn area(&self) -> f64 {
        (self.width * self.height) as f64
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!(
            "Drawing Rectangle at ({}, {}) with width {} and height {}",
            self.position.x, self.position.y, self.width, self.height
        );
    }
}