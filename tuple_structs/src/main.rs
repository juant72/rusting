struct Color(u8, u8, u8);
struct Point(u8, u8, u8);

fn get_y(p: Point) -> u8 {
    p.1
}

fn main() {
    let red = Color(255, 0, 0);
    println!("First value: {}", red.0);

    let coord = Point(4, 5, 6);
    let y = get_y(coord);
    println!("y is equal {}", y);
}
