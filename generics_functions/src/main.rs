fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("Biggest of 1 and 2 is {}", get_biggest(5, 2));
}
