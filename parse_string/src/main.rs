use std::io;
fn main() {
    let mut buffer = String::new();
    println!("Enter a message: ");
    io::stdin().read_line(&mut buffer);
    println!("Buffer is: {}", buffer);

    let number: i32 = buffer.trim().parse().unwrap();
    println!("Number + 1 is: {}", number + 1);
}
