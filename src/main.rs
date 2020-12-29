#[allow(warnings)]

mod lc564_closest_palindrome;

fn main() {
    println!("Hello, world!");
    dbg!(first_word(&"Hello world".to_string()));
    dbg!(first_word(&" Hello ".to_string()));
}


fn first_word(s: &String) -> &str {
    return match s.find(' ') {
        Some(i) => &s[0..i],
        None => &s[..]
    }
}

