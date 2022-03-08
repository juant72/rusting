#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 12.0022f64,
        height: 2u16,
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    let rect2 = Rectangle {
        width: 12u8,
        height: 2u8,
    };
    println!("perimeter is {}", rect2.get_perimeter());
}
