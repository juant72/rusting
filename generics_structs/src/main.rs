#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

fn main() {
    let rect = Rectangle {
        width: 12.0022f64,
        height: 2u16,
    };
    println!("rect is {:?}", rect);
}
