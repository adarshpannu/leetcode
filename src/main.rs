#[allow(warnings)]
mod flattenlist;

fn main() {
    let lst = vec!["hello", "world"];
    let mut s = "[".to_string();

    for (ix, e) in lst.iter().enumerate() {
        if ix > 0 {
            s.push_str(", ");
        }
        s.push_str("\"");
        s.push_str(e);
        s.push_str("\"");
    }
    s.push_str("]");

    println!("{}", s);
}