fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("first word is {word}");

    let string_literal = "hi world";
    let word = first_word(string_literal);
    println!("first word is {word}");

    s.clear();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
