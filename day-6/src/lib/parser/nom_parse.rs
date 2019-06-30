use super::Point;

use advent_utils::integer_parser;

use nom::{
    digit, map, map_res, named, separated_pair, tag,
    types::CompleteStr,
};

integer_parser!(parse_i32, i32);

named!(parse_point_raw<CompleteStr, Point>,
    map!(
        separated_pair!(parse_i32, tag!(", "), parse_i32),
        |(x, y)| Point::new(x, y)
    )
);

pub fn parse_point(s: &str) -> Result<Point, nom::ErrorKind> {
    parse_point_raw(CompleteStr(s))
        .map(|(_, point)| point)
        .map_err(|e| e.into_error_kind())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_raw() {
        assert_eq!(
            parse_point_raw(CompleteStr("123, 321")),
            Ok((CompleteStr(""), Point::new(123, 321)))
        )
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse_point("123, 312").unwrap(), Point::new(123, 312));

        assert!(parse_point("123,").is_err())
    }
}
