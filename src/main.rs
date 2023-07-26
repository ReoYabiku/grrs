fn main() {
    let word = String::from("hello world.");

    let first = first_word(&word);

    println!("the whole words are: {}", word);
    println!("the first word is: {}", first);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}