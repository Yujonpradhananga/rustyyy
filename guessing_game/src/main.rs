fn main() {
    let s2 = "nigga nigga";
    let word = first_word(&s2);
    println!("{}", word)
}

fn first_word(word: &str) -> &str {
    let bytes = word.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &word[0..i];
        }
    }
    &word[..]
}
