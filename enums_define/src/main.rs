#[derive(Debug)]
enum Shapes {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}
fn main() {
    let my_shape = Shapes::Rectangle(1.2, 3.4);
    println!("my shape is {:?}", my_shape);
}
