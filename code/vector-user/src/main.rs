use vector::space2d::Vector2D;

fn main() {
    let v1 = Vector2D::new(1.0, 2.0);
    let v2 = Vector2D::new(1.0, 2.0);
    let v3 = Vector2D::UNIT_X;

    println!("v1 == v2: {}", v1 == v2);
    println!("v1 + v2: {:?}", v1 + v2);
    println!("v3: {:?}", v3);
}
