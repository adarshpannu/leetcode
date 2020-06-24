//#[allow(warnings)]

// https://leetcode.com/problems/valid-number/

struct Solution {}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut parser = NumberParser::new(s.trim().to_string());
        match parser.parse() {
            Err(_) => false,
            Ok(_) => true,
        }
    }
}

pub struct NumberParser {
    chars: Vec<char>,
    next_idx: usize, // next character to read
}

#[derive(Debug, PartialEq)]
enum Token {
    Sign(i8),
    Dot,
    E,
    Number(u64),
    Unknown(char),
    EOF,
}

impl Token {
    fn is_eof(&self) -> bool {
        match *self {
            Token::EOF => true,
            _ => false,
        }
    }
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
            //println!("next() -> {}", ch);
            Some(ch)
        } else {
            None
        }
    }

    fn undo_next(&mut self) {
        assert!(self.next_idx > 0);
        self.next_idx -= 1;
        let ch = self.chars[self.next_idx];
        //println!("undo() -> {}", ch);
    }

    fn peek_token(&mut self) -> Token {
        let mut save_next_idx = self.next_idx;
        let token = self.next_token();
        self.next_idx = save_next_idx;
        token
    }

    fn next_token(&mut self) -> Token {
        while let Some(ch) = self.next() {
            match ch {
                //' ' => continue,
                '+' => return Token::Sign(1),
                '-' => return Token::Sign(-1),
                '.' => return Token::Dot,
                'e' => return Token::E,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    self.undo_next();
                    let num = self.parse_natural_number().unwrap();
                    return Token::Number(num);
                }
                _ => return Token::Unknown(ch),
            }
        }
        Token::EOF
    }

    fn to_digit(ch: Option<char>) -> Option<u64> {
        match ch {
            Some(digit) => {
                if (digit >= '0') && (digit <= '9') {
                    Some((digit as u64) - ('0' as u64))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn parse_natural_number(&mut self) -> Option<u64> {
        let mut number = vec![];
        let mut ch = self.next();

        while let Some(digit) = Self::to_digit(ch) {
            number.push(digit);
            ch = self.next();
            if ch == None {
                break;
            }
        }

        if ch != None {
            self.undo_next();
        }

        if number.len() > 0 {
            let mut retval = 0u64;
            for &digit in number.iter() {
                retval = (digit + retval * 10);
            }
            Some(retval)
        } else {
            None
        }
    }

    fn parse(&mut self) -> Result<(), String> {
        let mut token;
        let np = self;
        let retval: f64;

        // Parse optional leading sign +/-
        token = np.peek_token();
        let sign = if let Token::Sign(sign) = token {
            np.next_token();
            sign
        } else {
            1
        };

        // Parse mandatory base number
        token = np.peek_token();
        let base = match token {
            Token::Number(base) => {
                np.next_token();
                true
            },
            _ => false
        };

        // Parse optional dot + fractional part
        let mut fraction: bool = false;
        token = np.peek_token();
        if token == Token::Dot {
            np.next_token();

            // Parse optional fractional part
            token = np.peek_token();
            match token {
                Token::Number(_) => {
                    fraction = true;
                    np.next_token(); // consume token that we just peeked
                }
                _ => {}
            }
        }

        if !base && !fraction {
            return Err(format!("Unexpected token {:?}", token).to_string());
        }

        // Parse optional exponent
        token = np.peek_token();
        if (token == Token::E) {
            np.next_token(); // consume token that we just peeked

            // Parse optional plus/minus sign
            token = np.peek_token();
            if let Token::Sign(sign) = token {
                np.next_token(); // consume token that we just peeked
            }
            // Parse mandatory base number
            token = np.next_token();
            match token {
                Token::Number(base) => {}
                _ => return Err(format!("Unexpected character {:?}", token).to_string()),
            }
        }
        token = np.peek_token();
        if !token.is_eof() {
            return Err(format!("Unexpected character {:?}", token).to_string());
        }
        Ok(())
    }
}

#[test]
fn run() {
    let numbers = [
        "0",
        " 0.1 ",
        "abc",
        "1 a",
        "2e10",
        " -90e3   ",
        " 1e",
        "e3",
        " 6e-1",
        " 99e2.5 ",
        "53.5e93",
        " --6 ",
        "-+3",
        "95a54e53",
    ];

    let numbers = ["5. 6"];

    for numstr in numbers.iter() {
        //let mut np = Solution::new(numstr.to_string());
        //println!("Parse numstr = {}, result = {:?}", numstr, np.parse());
        println!(
            "Parse numstr = {}, result = {:?}",
            numstr,
            Solution::is_number(numstr.to_string())
        );
    }
}
