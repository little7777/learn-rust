fn main() {
    let s = String::from("Hello,world!");
    let word = first_world(&s);
    println!("{}", word)
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
