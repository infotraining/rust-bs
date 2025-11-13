mod shapes;

use shapes::prelude::{*};

fn create_shapes() -> Vec<Box<dyn Shape>> {
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();

    let rect = Rectangle {
        position: Point { x: 0, y: 0 },
        width: 10,
        height: 20
    };

    let circle = Circle {
        position: Point { x: 5, y: 5 },
        radius: 15
    };

    shapes.push(Box::new(rect));
    shapes.push(Box::new(circle));

    shapes
}

fn create_drawables() -> Vec<Box<dyn Drawable>> {
    let mut drawables: Vec<Box<dyn Drawable>> = Vec::new();

    let rect = Rectangle {
        position: Point { x: 0, y: 0 },
        width: 10,
        height: 20
    };

    let circle = Circle {
        position: Point { x: 5, y: 5 },
        radius: 15
    };

    let triangle = Triangle::new(
        Point { x: 1, y: 1 },
        Point { x: 4, y: 1 },
        Point { x: 1, y: 5 },
    );

    drawables.push(Box::new(rect));
    drawables.push(Box::new(circle));
    drawables.push(Box::new(triangle));

    drawables
}

fn create_drawable_shapes() -> Vec<Box<dyn DrawableShape>> {
    let mut shapes: Vec<Box<dyn DrawableShape>> = Vec::new();

    let rect = Rectangle {
        position: Point { x: 0, y: 0 },
        width: 10,
        height: 20
    };

    let circle = Circle {
        position: Point { x: 5, y: 5 },
        radius: 15
    };

    let triangle = Triangle::new(
        Point { x: 1, y: 1 },
        Point { x: 4, y: 1 },
        Point { x: 1, y: 5 },
    );

    shapes.push(Box::new(rect));
    shapes.push(Box::new(circle));
    shapes.push(Box::new(triangle));

    shapes
}

fn draw_shapes(shapes: &[&dyn Drawable]) {
    for shape in shapes.iter() {
        shape.draw();
    }
}

fn use_shapes() {
    let mut shapes = create_shapes();

    for shape in shapes.iter_mut() {
        shape.move_by(10, 10);
        println!("Shape area: {}", shape.area());
    }    
}

fn use_drawables() {
    let drawables = create_drawables();

    for drawable in drawables.iter() {
        drawable.draw();
    }  

    let drawable_refs: Vec<&dyn Drawable> = drawables.iter().map(|d| d.as_ref()).collect();
    draw_shapes(&drawable_refs);  
}   

fn use_drawable_shapes() {
    let mut drawable_shapes = create_drawable_shapes();

    for shape in drawable_shapes.iter_mut() {
        shape.move_by(20, 20);
        shape.draw();
    }    
}

fn main() {
    use_shapes();
    println!("{}", "-".repeat(30));
    use_drawables();
    println!("{}", "-".repeat(30));
    use_drawable_shapes();
}
