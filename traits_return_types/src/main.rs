use std::fmt;

fn get_displayable(choice: bool) -> impl fmt::Display {
    if choice {
        13
    } else {
        //"thirteen"
        14
    }
}

fn main() {
    println!("output {}", get_displayable(true));
}
