enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    if let Shape::Circle(radius) = shape {
        return 3.14 * radius * radius;
    }
    if let Shape::Square(side) = shape {
        return side * side;
    }
    if let Shape::Rectangle(length, width) = shape {
        return length * width;
    }
    0.0 // Default return value if no match (this will never be reached with the current enum)
}

fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    println!("Area of the circle: {}", calculate_area(circle));
    println!("Area of the square: {}", calculate_area(square));
    println!("Area of the rectangle: {}", calculate_area(rectangle));
}
