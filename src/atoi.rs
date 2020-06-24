#[allow(warnings)]

// https://leetcode.com/problems/valid-number/

pub struct NumberParser {
    chars: Vec<char>,
    next_idx: usize, // next character to read
}

enum Token {
    Plus,
    Minus,
    Dot,
    E,
    Number(i64),
    Unknown,
    EOF
}

// [-/+] [0-9]* [[.] [0-9]*] [ e [+/-] [0-9]* ]

impl NumberParser {
    fn new(str: String) -> NumberParser {
        let chars = str.chars().collect();
        NumberParser { chars, next_idx: 0 }
    }

    fn next(&mut self) -> Option<char> {
        if self.next_idx < self.chars.len() {
            let ch = self.chars[self.next_idx];
            self.next_idx += 1;
            println!("next() -> {}", ch);
            Some(ch)
        } else {
            None
        }
    }

    fn undo_next(&mut self) {
        assert!(self.next_idx > 0);
        self.next_idx -= 1;
        let ch = self.chars[self.next_idx];
        println!("undo() -> {}", ch);
    }

    fn next_token(&mut self) -> Token {
        let mut ch;

        loop {
            ch = self.next();
            if ch == None {
                break;
            }
            match ch.unwrap() {
                ' ' => { continue },
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {

                }
                _ => return Token::Unknown
            }
        }
        Token::EOF
    }

    fn parse_white_space(&mut self) {
        while (self.next() == Some(' ')) {}
        self.undo_next();
    }

    fn parse_plus_or_minus(&mut self) -> Option<char> {
        let ch = self.next();
        match ch {
            Some('+') | Some('-') => {
                self.undo_next();
                ch
            }
            _ => None,
        }
    }

    fn to_digit(ch: Option<char>) -> Option<u64> {
        match ch {
            Some(digit) => {
                if (digit >= '0') && (digit <= '9') {
                    Some((digit as u64) -  ('0' as u64))
                } else {
                    None
                }
            }
            _ => None
        }
    }

    fn parse_natural_number(&mut self) -> Option<u64> {
        let mut number = vec![];
        let mut ch = self.next();
        while let Some(digit) = Self::to_digit(ch) {
            number.push(digit);
            ch = self.next();
        }
        self.undo_next();
        if number.len() > 0 {
            let mut retval = 0u64;
            let mut multiplier = 1;
            number.reverse();
            for digit in number.iter() {
                retval += (*digit * multiplier);
                multiplier *= 10;
            }
            Some(retval)
        } else {
            None
        }
    }

    pub fn is_number() -> bool {
        false
    }
}

#[test]
fn run() {
    let mut np = NumberParser::new(" 123.".to_string());

    println!("{:?}", np.parse_white_space());
    println!("{:?}", np.parse_natural_number());

}
