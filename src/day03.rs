use std::str::Chars;

use crate::input;

pub fn run() {
    let lines = input::get_lines("day03");

    println!("Part1: {:?}", sum_of_mul(&lines));
}

fn sum_of_mul(lines: &[String]) -> i32 {
    let input = lines.join("");
    let mut chars = input.chars();
    let mut total = 0;

    let mut searching = true;
    while searching {
        if find_mul_keyword(&mut chars) {
            if let (Some(x), Some(',')) = get_number(&mut chars) {
                if let (Some(y), Some(')')) = get_number(&mut chars) {
                    total += x * y;
                }
            }
        } else {
            searching = false;
        }
    }
    total
}

fn find_mul_keyword(chars: &mut Chars) -> bool {
    loop {
        let mut n = chars.next();
        while n.is_some() {
            let c = n.unwrap();
            if c == 'm' {
                break;
            }
            n = chars.next();
        }
        if n.is_none() {
            return false;
        }
        if find_char(chars, 'u') && find_char(chars, 'l') && find_char(chars, '(') {
            return true;
        }
    }
}

fn get_number(chars: &mut Chars) -> (Option<i32>, Option<char>) {
    let mut found = String::new();

    let mut peekable = chars.peekable();

    let next;

    loop {
        let p = peekable.peek().cloned();
        if p.unwrap_or_default().is_digit(10) {
            found.push(peekable.next().unwrap());
        } else {
            next = peekable.peek().cloned();
            break;
        }
    }
    (found.parse::<i32>().ok(), next)
}

fn find_char(chars: &mut Chars, c: char) -> bool {
    let result = chars.next();
    result == Some(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = vec![
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string(),
        ];

        assert_eq!(sum_of_mul(&example), 161);
    }

    #[test]
    fn multiline_example() {
        let example = vec![
            "^<when()]'>mul(721,815)who()&[how()mul(990,467)select()<".to_string(),
            "*,/~):^[mul(802,939~;!:@mul(656,831)}]?~mul(92,389)/".to_string(),
        ];

        assert_eq!(sum_of_mul(&example), 1_630_869);
    }
}
