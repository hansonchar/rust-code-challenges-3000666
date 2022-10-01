use std::{cmp::Ordering, str::FromStr};

#[allow(dead_code)]
struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum IsbnError {
    TooLong,
    TooShort,
    InvalidCheckDigit,
}

impl FromStr for Isbn {
    type Err = IsbnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits: Vec<u8> = s
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        match digits.len().cmp(&13) {
            Ordering::Less => return Err(IsbnError::TooShort),
            Ordering::Greater => return Err(IsbnError::TooLong),
            _ => (),
        };

        let actual = digits[11];
        let expected = calculate_check_digit(&digits[0..12]);
        if actual != expected {
            return Err(IsbnError::InvalidCheckDigit);
        }
        Ok(Isbn {
            raw: s.to_string(),
            digits,
        })
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let mut odd = false;
    let sum = digits.iter().fold(0, |acc, n| {
        let r = (acc + if odd { n * 3 } else { *n }) % 10;
        odd = !odd;
        r
    });
    (10 - sum) % 10
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
