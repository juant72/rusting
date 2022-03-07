use rand::prelude::*;
use std::io;

fn main() {
    println!("Quantity of numbers?");
    let mut quantity = String::new();
    io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read line");
    let quantity: u32 = quantity.trim().parse().expect("Please type a number!");
    println!("Quantity of numbrers: {}", quantity);

    // Generate random number
    let rng = thread_rng().gen_range(1..quantity + 1);
    //println!("Random number: {}", rng);

    while true {
        println!("Guess the number!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        if guess == rng {
            println!("You win!");
            break;
        } else {
            println!("You lose!");
        }
    }
}
