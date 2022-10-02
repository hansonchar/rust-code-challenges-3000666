use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb(u8, u8, u8);

trait RgbChannels {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.0
    }
    fn g(&self) -> u8 {
        self.1
    }
    fn b(&self) -> u8 {
        self.2
    }
}

#[derive(Debug)]
enum RgbError {
    ParseError(ParseIntError),
    InputLengthError,
    MissingHashPrefix,
}

const BASE: i64 = 256;

impl FromStr for Rgb {
    type Err = RgbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 {
            Err(RgbError::InputLengthError)
        } else if !s.starts_with('#') {
            Err(RgbError::MissingHashPrefix)
        } else {
            let val = i64::from_str_radix(&s[1..7], 16).map_err(RgbError::ParseError)?;
            Ok(Rgb(
                (val % BASE) as u8,
                ((val / BASE) % BASE) as u8,
                ((val / BASE / BASE) % BASE) as u8,
            ))
        }
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    //
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}
