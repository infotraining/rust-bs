use std::fmt::Debug;

#[cxx::bridge(namespace = "Shapes")]
mod ffi {

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("cxx-demo/include/rectangle.hpp");

        type Rectangle;

        fn new_rectangle(width: f64, height: f64) -> UniquePtr<Rectangle>;
        fn area(self: &Rectangle) -> f64;
        fn width(self: &Rectangle) -> f64;
        fn height(self: &Rectangle) -> f64;
        fn inflate(self: Pin<&mut Rectangle>, factor: f64);
        #[Self = "Rectangle"]
        fn id() -> &'static CxxString;
    }
}

impl Debug for ffi::Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rectangle")
            .field("width", &self.width()) 
            .field("height", &self.height()) 
            .finish()
    }
}

fn main() {
    let mut rect = ffi::new_rectangle(3.0, 4.0);
    let area = rect.area();
    println!("Area of rectangle: {}", area);

    rect.pin_mut().inflate(2.0);
    let new_area = rect.area();
    println!("New area of rectangle after inflation: {}", new_area);
    let id = ffi::Rectangle::id();
    println!("Rectangle ID: {}", id);

    let rectanles = vec![
        ffi::new_rectangle(1.0, 2.0),
        ffi::new_rectangle(3.0, 4.0),
        ffi::new_rectangle(5.0, 6.0),
        ffi::new_rectangle(7.0, 8.0),
    ];

    for (i, rectangle) in rectanles.iter().enumerate() {
        println!("{}. {:?}; Area = {:.2}", i+1, rectangle, rectangle.area());
    }
}
