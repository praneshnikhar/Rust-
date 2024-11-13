enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    if let Shape::Circle(radius) = shape{
        println!("Circle with radius; {}", radius);
    }else if let Shape::Square(side) = shape{
        println!("Square with side {}", side);
    }else if let Shape::Rectangle(width, height)= shape{
        println!("Rectangle with width: {} and height: {}", width , height );
    }

    let ans = match shape{
        Shape::Circle(radius) => 3.14*radius*radius,
        Shape::Rectangle(width, height)=>{
            width*height
        },
        Shape::Square(side) => side * side
    } ;
    return ans;
        
    }
    
    // match shape{
    //     Shape::Circle(radius)=> std::f64::consts::PI*radius *radius,
    //     Shape::Square(side_length)=>side_length*side_length,
    //     Shape::Rectangle(width,height ) => width * height,
    // }



fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    println!("Area of the circle: {}", calculate_area(circle));
    println!("Area of the square: {}", calculate_area(square));
    println!("Area of the rectangle: {}", calculate_area(rectangle));
}
