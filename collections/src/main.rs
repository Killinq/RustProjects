mod latin;
mod mode;

use std::io;

fn main() {
    //let mut vec: Vec<i32> = Vec::new();
    //mode::add(&mut vec);
    println!("Type a word to convert to piglatin:");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("failed");
    word = String::from(word.trim());
    latin::piglatin(&mut word);
}
