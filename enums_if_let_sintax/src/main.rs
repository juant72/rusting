fn main() {
    let number = Some(13);
    match number {
        Some(13) => println!("number is thirteen"),
        _ => (),
    };

    if let Some(13) = number {
        println!("number is thirteen");
    }
}
