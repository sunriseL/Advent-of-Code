use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::char;

#[derive(Debug)]
enum Token {
    Mul,
    LeftBracket,
    RightBracket,
    Comma,
    Number(u32),
    Invalid,
    EOF,
}

pub struct Lexer<'a> {
    input: &'a Vec<char>,
    current_iter: std::slice::Iter<'a, char>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a Vec<char>) -> Self {
        let mut lexer = Lexer {
            input,
            current_iter: input.iter(),
            current_char: None,
        };
        lexer.advance();
        lexer
    }
    pub fn advance(&mut self) {
        self.current_char = self.current_iter.next().copied();
    }
}

impl<'a> Lexer<'a> {
    pub fn next_token(&mut self) -> Token {
        match self.current_char {
            Some(',') => {
                self.advance();
                Token::Comma
            }
            Some('(') => {
                self.advance();
                Token::LeftBracket
            }
            Some(')') => {
                self.advance();
                Token::RightBracket
            }
            Some(c) if c.is_digit(10) => self.number(),
            Some(c) if c.is_alphabetic() => self.identifier(),
            None => Token::EOF,
            _ => {
                self.advance();
                Token::Invalid
            }
        }
    }

    fn number(&mut self) -> Token {
        let mut number: u32 = 0;
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                number = number * 10 + (c.to_digit(10).unwrap());
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(number)
    }

    fn identifier(&mut self) -> Token {
        let mut identifier_str = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphabetic() {
                identifier_str.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match identifier_str.as_str() {
            "mul" => Token::Mul,
            _ => Token::Invalid,
        }
    }
}

#[derive(PartialEq)]
enum Status {

    Idle,
    WithMul,
    WithLeftBracket,
    WithLeftNumber,
    WithComma,
    WithRightNumber,

}

pub fn process(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let input: Vec<char> = reader.bytes().map(|b| char::from(b.unwrap())).collect();
    let mut lexer = Lexer::new(&input);
    let mut left_number: Option<u32> = None;
    let mut right_number: Option<u32> = None;
    let mut status: Status = Status::Idle;
    let mut result: i64 = 0;
    while let token = lexer.next_token() {
        match token {
            Token::EOF => break,
            Token::Invalid => {
                status = Status::Idle;
            }
            Token::Mul => {
                match status {
                    Status::Idle => status = Status::WithMul,
                    _ => status = Status::Idle,
                }
            }
            Token::LeftBracket => {
                match status {
                    Status::WithMul => status = Status::WithLeftBracket,
                    _ => status = Status::Idle,
                }
            }
            Token::Number(number) if number < 1000 => {
                if number < 1000 && status == Status::WithLeftBracket {
                    left_number = Some(number);
                    status = Status::WithLeftNumber;
                } else if number < 1000 && status == Status::WithComma {
                    right_number = Some(number);
                    status = Status::WithRightNumber;
                } else {
                    status = Status::Idle;
                }
            }
            Token::Comma => {
                match status {
                    Status::WithLeftNumber => status = Status::WithComma,
                    _ => status = Status::Idle,
                }
            }
            Token::RightBracket => {
                match status {
                    Status::WithRightNumber => {
                        result += i64::from(left_number.unwrap() * right_number.unwrap());
                        left_number = None;
                        right_number = None;
                        status = Status::Idle;
                    }
                    _ => {
                        status = Status::Idle;
                    }
                }
            }
            _ => status = Status::Idle,
        }
    }
    result
}
