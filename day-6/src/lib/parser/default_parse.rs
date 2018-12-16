use std::{num::ParseIntError, str::FromStr};

use super::Point;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    TooFewArgs,
    ParseError(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::ParseError(err)
    }
}

pub fn parse_point(s: &str) -> Result<Point, ParseError> {
    let mut splitted = s.split(", ");

    let x = splitted.next().ok_or(ParseError::TooFewArgs)?.parse()?;
    let y = splitted.next().ok_or(ParseError::TooFewArgs)?.parse()?;

    Ok(Point::new(x, y))
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_point(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
        assert_eq!(parse_point("123, 321"), Ok(Point::new(123, 321)));
        assert!(parse_point("123,").is_err());
    }

    #[test]
    fn test_from_str() {
        assert_eq!("123, 321".parse(), Ok(Point::new(123, 321)));
        assert!("123,s".parse::<Point>().is_err());
    }
}
