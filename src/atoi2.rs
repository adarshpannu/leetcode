//#[allow(warnings)]

// https://leetcode.com/problems/valid-number/
// Parse a number: [-/+] [0-9]* [[.] [0-9]*] [ e [+/-] [0-9]* ]
use std::collections::HashMap;

struct Solution {}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum State {
    Start,
    Base,
    Fractional,
    Exponent,
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
            ((Start, Sign), Base),
            ((Start, Digit), Base),
            ((Start, Dot), Fractional),
            ((Base, Digit), Base),
            ((Base, Dot), Fractional),
            ((Base, EOF), End),
            ((Fractional, Digit), Fractional),
            ((Fractional, E), Exponent),
            ((Fractional, EOF), End),
            ((Exponent, Sign), ExponentBase),
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
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Token::Digit,
                '.' => Token::Dot,
                'e' | 'E' => Token::E,
                _ => panic!("Illegal character found: {}", ch),
            }
        }
    }

    fn parse(&mut self) -> Result<(), &'static str> {
        let mut token;
        let mut state = Start;
    
        loop {
            token = self.next_token();
            let key = (state, token);
            let new_state = self.transitions.get(&key);
            if new_state.is_none() {
                return Err("Illegal state transition.");
            }
            let new_state = new_state.unwrap();
            println!("({:?}, {:?}) -> {:?}", state, token, new_state);
            state = *new_state;
            if token == Token::EOF {
                break; 
            }
        }
        if state != End {
            return Err("Unexpected end of input.");
        }
        Ok(())
    }    
}

#[test]
fn run() {
    println!("\nRESULT = {:?}\n", Solution::is_number(".e9".to_string()));
}
