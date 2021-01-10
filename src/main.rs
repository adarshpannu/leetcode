#[allow(warnings)]
mod lc1657_close_strings;

fn main() {
    println!("Hello, world!");
}

fn largest<T>(lst: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &lst[0];
    for e in lst {
        if e > largest {
            largest = e
        }
    }
    largest
}

struct S<T> {
    e: T
}

#[test]
fn test() {
    let lst = vec![10, 0, -1, 98, 11, 8];
    dbg!(largest(&lst));

    let lst: Vec<i8> = vec![10, 0, -1, 98, 11, 8];
    dbg!(largest(&lst));

}
