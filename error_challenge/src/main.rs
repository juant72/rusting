use rand::prelude::*;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("I'm thinking of a number between 1 and 100.");
    println!("Guess the number");
    loop {
        let mut buffer = String::new();
        let guess = match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<u32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("That's not a number!");
                    continue;
                }
            },
            Err(_) => {
                println!("That's not a number!");
                continue;
            }
        };

        if guess > secret_number {
            println!("\n {} Too high! ", guess);
        } else if guess < secret_number {
            println!("\n {} Too low!", guess);
        } else {
            println!("You win! {}", guess);
            break;
        }
    }
}
