use crate::shapes::traits::{Shape, Drawable};
use crate::shapes::point::Point;

pub struct Triangle {
    pub points: [Point; 3]
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Triangle {
            points: [p1, p2, p3]
        }
    }
}

impl Shape for Triangle {
    fn move_by(&mut self, dx: i32, dy: i32) {
        for point in &mut self.points {
            point.x += dx;
            point.y += dy;
        }
    }

    fn area(&self) -> f64 {
        let a = ((self.points[0].x - self.points[1].x).pow(2)
            + (self.points[0].y - self.points[1].y).pow(2)) as f64;
        let b = ((self.points[1].x - self.points[2].x).pow(2)
            + (self.points[1].y - self.points[2].y).pow(2)) as f64;
        let c = ((self.points[2].x - self.points[0].x).pow(2)
            + (self.points[2].y - self.points[0].y).pow(2)) as f64;

        let s = (a + b + c) / 2.0;
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
}

impl Drawable for Triangle {
    fn draw(&self) {
        println!(
            "Drawing Triangle with points ({}, {}), ({}, {}), ({}, {})",
            self.points[0].x, self.points[0].y,
            self.points[1].x, self.points[1].y,
            self.points[2].x, self.points[2].y
        );
    }
}