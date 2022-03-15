#[derive(Debug)]
enum Shapes {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shapes {
    fn get_perimeter(&self) -> f64 {
        match self {
            Shapes::Circle(radius) => 2.0 * std::f64::consts::PI * radius,
            Shapes::Rectangle(width, height) => 2.0 * (width + height),
            Shapes::Triangle(a, b, c) => a + b + c,
        }
    }
}

fn main() {
    let my_shape = Shapes::Rectangle(1.2, 3.4);
    println!("my shape is {:?}", my_shape);

    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {}", perimeter);
}
