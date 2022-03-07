use std::env;
fn main() {
    if env::args().count() <= 2 {
        println!("Please provide at least 2 arguments");
        return;
    }

    for (index, arguments) in env::args().enumerate() {
        println!("argument {} is : {}", index, arguments);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is : {}", arg2);
}
