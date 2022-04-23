pub fn piglatin(word: &mut String) {
    let vowels = ["a", "e", "i", "o", "u"];
    if !vowels.contains(&&word[0..1]) {
        let first = word.remove(0);
        word.push(first);
    }
    word.push_str("ay");
    println!("Result: ");
    print!("{}", word);
}
