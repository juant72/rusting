use std::fs;

fn main() {
    let contents =
        fs::read_to_string("the_ultimate_question.txt").expect("Please provide a valid file path");
    println!("contents is {:?}", contents);
}
