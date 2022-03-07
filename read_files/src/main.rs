use std::fs;

fn main() {
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("The content is {}", contents);

    for line in contents.lines() {
        println!("Line {}", line);
    }

    //Binary contents
    let contents = fs::read("planets.txt").unwrap();
    println!("The content is {:?}", contents);
}
