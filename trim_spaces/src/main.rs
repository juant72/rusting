fn main() {
    let test1 = String::from("We need more space.");
    assert_eq!(trim_spaces(test1), String::from("We need more space."));
}

fn trim_spaces(s: String) -> String {
    let mut result = String::new();
    result
}
