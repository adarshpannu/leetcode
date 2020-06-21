pub fn run() {
    let a = vec![
        String::from("A11"),
        String::from("A12"),
        String::from("A13"),
        String::from("A21"),
        String::from("A22"),
        String::from("A23"),
        String::from("A24"),
        String::from("B31"),
        String::from("B32"),
    ];

    let excl_strings = vec![String::from("C2"), String::from("A13"), String::from("A2")];

    let result: Vec<&String> = a
        .iter()
        .filter(|s| {
            let mut exclude = false;
            for search_string in &excl_strings {
                if s.starts_with(search_string) {
                    exclude = true;
                    break;
                } else {
                    exclude = false;
                };
            }
            !exclude
        })
        .collect();

    println!("{:?}", result);
}

use std::collections::HashSet;

pub fn run_mine() {
    let a = vec![
        String::from("A11"),
        String::from("A12"),
        String::from("A13"),
        String::from("A21"),
        String::from("A22"),
        String::from("A23"),
        String::from("A24"),
        String::from("B31"),
        String::from("B32"),
    ];

    let excl_strings = vec![String::from("C2"), String::from("A13"), String::from("A2")];

    let mut excl_strings_set: HashSet<&String> = HashSet::new();

    for s in excl_strings.iter() {
        excl_strings_set.insert(s);
    }

    let result: Vec<&String> = a.iter().filter(|&e| ! excl_strings_set.contains(e)).collect();

    println!("{:?}", result);

}
