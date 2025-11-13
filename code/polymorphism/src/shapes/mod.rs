pub mod traits;
pub mod circle;
pub mod rectangle;
pub mod point;
pub mod triangle;

pub mod prelude {
    pub use super::traits::{Shape, Drawable, DrawableShape};
    pub use super::circle::Circle;
    pub use super::rectangle::Rectangle;
    pub use super::point::Point;
    pub use super::triangle::Triangle;
}