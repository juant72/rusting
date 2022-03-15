use std::fs;
use std::io;

fn main() {
    let results = fs::read_to_string("the_ultimate_question.txt");

    let contents = match results {
        Ok(contents) => contents,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                panic!("Could not find the file specified");
            }
            io::ErrorKind::PermissionDenied => {
                panic!("Could not open the file due to permissions");
            }
            _ => {
                panic!("There was an error reading the file {:?}", e);
            }
        },
    };
    println!("contents is {:?}", contents);
}
