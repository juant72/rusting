#[derive(Debug)]
enum Shapes {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}
fn main() {
    let my_shape = Shapes::Rectangle(1.2, 3.4);
    println!("my shape is {:?}", my_shape);

    match my_shape {
        Shapes::Circle(radius) => println!("circle with radius {}", radius),
        Shapes::Rectangle(width, height) => {
            println!("rectangle with width {} and height {}", width, height)
        }
        Shapes::Triangle(a, b, c) => println!("triangle with sides {} {} {}", a, b, c),
    }
}
