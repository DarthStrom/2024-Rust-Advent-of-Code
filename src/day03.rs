use crate::input;
use regex::Regex;

pub fn run() {
    let lines = input::get_lines("day03");

    println!("Part1: {:?}", sum_of_mul(&lines.join("")));
    println!("Part2: {:?}", sum_of_enabled_mul(&lines.join("")));
}

#[derive(Debug, PartialEq)]
enum Token {
    Mul,
    LeftParen,
    RightParen,
    Comma,
    Do,
    DoNot,
    Number(i32),
    EOF,
}

struct Scanner {
    source: String,
    start: usize,
    current: usize,
}

// Leaving this around. Let's see if they make us do more complicated work than regex can handle.
// It doesn't work quite right yet, but can be a good starting point.
impl Scanner {
    fn new(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Some(token) => tokens.push(token),
                None => (),
            }
        }

        tokens.push(Token::EOF);

        println!("{:?}", tokens);

        tokens
    }

    fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance();

        match c {
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some(',') => Some(Token::Comma),
            Some('m') => {
                if self.next_is('u') && self.next_is('l') {
                    Some(Token::Mul)
                } else {
                    None
                }
            }
            Some('d') => {
                if self.next_is('o') {
                    if self.next_is('n') && self.next_is('\'') && self.next_is('t') {
                        Some(Token::DoNot)
                    } else {
                        Some(Token::Do)
                    }
                } else {
                    None
                }
            }
            Some(c) => {
                if c.is_digit(10) {
                    self.number()
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn number(&mut self) -> Option<Token> {
        while self.peek().is_digit(10) {
            self.advance();
        }

        let value = self.source[self.start..self.current]
            .parse::<i32>()
            .unwrap();

        Some(Token::Number(value))
    }

    fn next_is(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current) != Some(expected) {
            false
        } else {
            self.current += 1;

            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap_or('\0')
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        let result = self.source.chars().nth(self.current);
        self.current += 1;

        result
    }
}

pub fn sum_of_mul(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for (_, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
    }
    sum
}

pub fn sum_of_enabled_mul(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(d)(o)(?:n't)?\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for (s, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        match s {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap() * enabled as i32,
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = vec![
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string(),
        ];

        assert_eq!(sum_of_mul(&example.join("")), 161);
    }

    #[test]
    fn multiline_example() {
        let example = vec![
            "^<when()]'>mul(721,815)who()&[how()mul(990,467)select()<".to_string(),
            "*,/~):^[mul(802,939~;!:@mul(656,831)}]?~mul(92,389)/".to_string(),
        ];

        assert_eq!(sum_of_mul(&example.join("")), 1_630_869);
    }

    #[test]
    fn part2_example() {
        let example = vec![
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string(),
        ];

        assert_eq!(sum_of_enabled_mul(&example.join("")), 48);
    }
}
