//#[allow(warnings)]

// https://leetcode.com/problems/valid-number/
// Parse a number: [-/+] [0-9]* [[.] [0-9]*] [ e [+/-] [0-9]* ]
use std::collections::HashMap;

struct Solution {}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum State {
    Start,
    Base0,
    Base,
    Fractional0,
    Fractional,
    Exponent,
    ExponentBase0,
    ExponentBase,
    End,
}
use State::*;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Token {
    Sign,
    Digit,
    E,
    Dot,
    Unknown,
    EOF,
}
use Token::*;

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut parser = NumberParser::new(s);
        parser.parse() == Ok(())
    }
}

struct NumberParser {
    chars: Vec<char>,
    next_idx: usize,
    transitions: HashMap<(State, Token), State>,
}

impl NumberParser {
    fn new(str: String) -> NumberParser {
        let transitions = vec![
            ((Start, Sign), Base0),
            ((Start, Digit), Base),
            ((Start, Dot), Fractional0),
            ((Base0, Digit), Base),
            ((Base0, Dot), Fractional0),
            ((Base, Digit), Base),
            ((Base, E), Exponent),
            ((Base, Dot), Fractional0),
            ((Base, EOF), End),
            ((Fractional0, Digit), Fractional),
            ((Fractional0, EOF), End),
            ((Fractional0, E), Exponent),
            ((Fractional, Digit), Fractional),
            ((Fractional, E), Exponent),
            ((Fractional, EOF), End),
            ((Exponent, Sign), ExponentBase0),
            ((ExponentBase0, Digit), ExponentBase),
            ((Exponent, Digit), ExponentBase),
            ((ExponentBase, Digit), ExponentBase),
            ((ExponentBase, EOF), End),
        ];

        let transitions: HashMap<_, _> = transitions.into_iter().collect();

        NumberParser { chars: str.trim().chars().collect(), next_idx: 0, transitions }
    }

    fn next_token(&mut self) -> Token {
        if self.next_idx >= self.chars.len() {
            Token::EOF
        } else {
            let ch = self.chars[self.next_idx];
            self.next_idx += 1;
            match ch {
                '+' | '-' => Token::Sign,
                '0'..='9' => Token::Digit,
                '.' => Token::Dot,
                'e' | 'E' => Token::E,
                _ => Token::Unknown,
            }
        }
    }

    fn parse(&mut self) -> Result<(), &'static str> {
        let mut token;
        let mut state = Start;
        let mut seen_base = false;
        let mut seen_fractional = false;

        loop {
            token = self.next_token();
            if token == Token::Unknown {
                return Err("Unexpected character/token found.");
            }

            let key = (state, token);
            let new_state = self.transitions.get(&key);
            if new_state.is_none() {
                return Err("Illegal state transition.");
            }
            let new_state = new_state.unwrap();
            //println!("({:?}, {:?}) -> {:?}", state, token, new_state);
            state = *new_state;
            match state {
                Base => seen_base = true,
                Fractional => seen_fractional = true,
                _ => {}
            }
            if token == Token::EOF {
                break; 
            }
        }
        if state != End {
            return Err("Unexpected end of input.");
        }
        if !seen_base && !seen_fractional {
            return Err("Malforned number: neither base nor fractional part exists.");
        }
        Ok(())
    }    
}

#[test]
fn run() {
    println!("\nRESULT = {:?}\n", Solution::is_number("46.e3".to_string()));
}
