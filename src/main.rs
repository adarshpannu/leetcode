#![allow(warnings)]

mod sets;

const SLASH: u8 = '/' as u8;

fn clean_url(str: &String) -> String {
    let mut last_ch: u8 = 0;

    let buf = str
        .as_bytes()
        .iter()
        .cloned()
        .filter(|ch| {
            let skip = (*ch == SLASH) && (last_ch == SLASH);
            last_ch = *ch;
            !skip
        })
        .collect::<Vec<u8>>();

    String::from_utf8(buf).unwrap()
}

#[derive(Debug)]
struct STKT {
    str: String,
}

fn foo(s: String) -> STKT {
    let str = format!("sss");
    let strt = STKT { str };

    dbg!(&strt.str);
    dbg!(1 * 9);

    return strt;
}

const DAYS_STRINGS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

fn day_string0(day_nbr: i32) -> &'static str {
    match day_nbr {
        1 => &"first",
        2 => &"second",
        3 => &"third",
        4 => &"fourth",
        5 => &"fifth",
        6 => &"sixth",
        7 => &"seventh",
        8 => &"eighth",
        9 => &"ninth",
        10 => &"tenth",
        11 => &"eleventh",
        12 => &"twelfth",
        _ => panic!("day_nbr not supported."),
    }
}

fn day_string(day_nbr: i32) -> &'static str {
    DAYS_STRINGS[day_nbr as usize]
}

fn main() {
    println!("day_string = {}", day_string(3));
}
