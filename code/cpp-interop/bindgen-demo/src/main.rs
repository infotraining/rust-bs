mod bindings;
use bindings::{rectangle_area, rectangle_free, rectangle_new};

pub struct RectangleHandle {
    ptr: *mut bindings::Rectangle,
}

impl RectangleHandle {
    pub fn new(width: f64, height: f64) -> Self {
        let ptr = unsafe { bindings::rectangle_new(width, height) };
        assert!(!ptr.is_null(), "Failed to create Rectangle");
        Self { ptr }
    }

    pub fn area(&self) -> f64 {
        unsafe { bindings::rectangle_area(self.ptr) }
    }
}

impl Drop for RectangleHandle {
    fn drop(&mut self) {
        unsafe {
            bindings::rectangle_free(self.ptr);
        }
    }
}

fn unsafe_usage_of_rectangle() {
    unsafe {
        let rect = rectangle_new(3.0, 4.0);
        let area = rectangle_area(rect);
        println!("Area: {}", area);
        rectangle_free(rect);
    }
}


fn main() {
    unsafe_usage_of_rectangle();

    let rectangle = RectangleHandle::new(5.0, 6.0);
    let area = rectangle.area();
    println!("Area using safe wrapper: {}", area);
}
