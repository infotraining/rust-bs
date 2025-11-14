use std::any::Any;

pub trait Shape: Any {
    fn move_by(&mut self, dx: i32, dy: i32);
    fn area(&self) -> f64;
    fn as_any(&self) -> &dyn Any;
}

pub trait Drawable {
    fn draw(&self);
}

pub trait DrawableShape: Shape + Drawable {} // supertrait combining Shape and Drawable

impl<T> DrawableShape for T where T: Shape + Drawable {} // blanket implementation