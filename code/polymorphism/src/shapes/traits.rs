pub trait Shape {
    fn move_by(&mut self, dx: i32, dy: i32);
    fn area(&self) -> f64;
}

pub trait Drawable {
    fn draw(&self);
}

pub trait DrawableShape: Shape + Drawable {} // supertrait combining Shape and Drawable

impl<T> DrawableShape for T where T: Shape + Drawable {} // blanket implementation