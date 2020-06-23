#![allow(warnings)]

//   [[1], 2, [[3, 4], 5], [[[]]], [[[6]]], 7, 8, []]
use std::fmt;

#[derive(Debug)]
enum Elem {
    int(i32),
    list(Box<List>),
}

struct List {
    elements: Vec<Elem>,
}

impl List {
    fn new() -> List {
        List { elements: vec![] }
    }

    fn pushi(&mut self, int: i32) {
        self.elements.push(Elem::int(int));
    }

    fn pushl(&mut self, list: List) {
        self.elements.push(Elem::list(Box::new(list)));
    }

    fn len(&self) -> usize {
        self.elements.len()
    }
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for elem in &self.elements {
            match *elem {
                Elem::int(int) => write!(f, " {} ", int)?,
                Elem::list(ref list) => write!(f, " {:?} ", *list)?,
            }
        }
        write!(f, "]")
    }
}

#[test]
fn run() {
    let s = "[[1], 2, [[3, 4], 5], [[[]]], [[[6]]], 7, 8, []]";
    //let s = "[0, [1, 2]]";
    //let s = "[1]";

    let mut cur_list;
    let mut stack: Vec<List>;

    cur_list = List::new();
    stack = vec!();

    for ch in s.chars() {
        match ch {
            ' ' | ',' => { continue }
            '[' => {
                stack.push(cur_list);
                cur_list = List::new();
            }
            ']' => {
                if stack.len() > 0 {
                    let mut parent_list = stack.pop().unwrap();
                    parent_list.pushl(cur_list);
                    cur_list = parent_list;
                }
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let elem = ch as i32 - '0' as i32;
                cur_list.pushi(elem);
            }
            _ => panic!("Unexpected character: {}", ch),
        }
        //println!("Added {:?}, cur_list = {:?}, stack = {:?}", ch, cur_list, stack);
    }
    println!("cur_list: {:?}", cur_list);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        println!("Hello");

        super::run();
    }
}