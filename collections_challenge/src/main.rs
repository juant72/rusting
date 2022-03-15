use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let contents = match env::args().nth(1) {
        Some(f) => match fs::read_to_string(f) {
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprintln!("Could not read : {}", e);
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Program require an argument <file_path>");
            std::process::exit(2);
        }
    };
    let all_words = contents.split_whitespace().collect::<Vec<&str>>();
    //count how many times each word appears
    let mut word_count: HashMap<&str, u32> = HashMap::new();
    for word in all_words.iter() {
        *word_count.entry(word).or_insert(0) += 1;
    }
    //determine the most common word
    let mut most_common_word = "";
    let mut most_common_count = 0;
    for (word, count) in word_count.iter() {
        if *count > most_common_count {
            most_common_word = word;
            most_common_count = *count;
        }
    }
    println!("The most common word is {}", most_common_word);
}
