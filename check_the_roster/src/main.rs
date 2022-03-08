use std::env::args;
use std::fs;

fn main() {
    let archive_path = args().nth(1).expect("Please provide an archive path");
    let person = args().nth(2).expect("Please provide a person name");
    let moonwalkers = fs::read_to_string(archive_path).expect("File not found");
    for line in moonwalkers.lines() {
        if line.contains(&person) {
            println!("Contains the moonwlaker {}", line);
            return;
        }
    }
    println!("No contains the moonwlaker");
}
