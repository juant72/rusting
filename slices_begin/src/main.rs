fn main() {
    let message = String::from("Greetings from Earth..");
    //let last_word = &message[15..15 + 5];
    let last_word = &message[15..];
    println!("Last word {}", last_word);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets = &planets[..4];
    println!("inner planets {:?}", inner_planets);
}
