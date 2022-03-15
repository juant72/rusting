use std::fs;
use std::io;

fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    let mut s1 = match fs::read_to_string(f1) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let s2 = match fs::read_to_string(f2) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}

fn main() {
    let result = read_and_combine("the_ultimate_question.txt", "the_ultimate_answer.txt");
    match result {
        Ok(s) => println!("result is {}", s),
        Err(e) => println!("error is {:?}", e),
    }
}
