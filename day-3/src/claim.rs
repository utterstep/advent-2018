use std::str::FromStr;

use nom::{call, do_parse, error_position, map_res, named, tag, types::CompleteStr};

pub(crate) type NomError = nom::ErrorKind;

#[derive(Debug, PartialEq)]
pub struct Claim {
    pub number: i32,
    pub top: usize,
    pub left: usize,
    pub width: usize,
    pub height: usize,
}

impl FromStr for Claim {
    type Err = NomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_claim(s)
    }
}

named!(number_i32<CompleteStr, i32>,
    map_res!(
        nom::digit,
        |i: CompleteStr| str::parse(i.0)
    )
);

named!(number_usize<CompleteStr, usize>,
    map_res!(
        nom::digit,
        |i: CompleteStr| str::parse(i.0)
    )
);

named!(claim_int<CompleteStr, Claim>,
    do_parse!(
        tag!("#") >>
        number: number_i32 >>
        tag!(" @ ") >>
        left: number_usize >>
        tag!(",") >>
        top: number_usize >>
        tag!(": ") >>
        width: number_usize >>
        tag!("x") >>
        height: number_usize >>
        (Claim {
            number,
            top,
            left,
            width,
            height,
        })
    )
);

pub(crate) fn parse_claim(input: &str) -> Result<Claim, nom::ErrorKind> {
    claim_int(CompleteStr(input))
        .map(|(_, result)| result)
        .map_err(|e| e.into_error_kind())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_usize() {
        assert_eq!(
            number_usize(CompleteStr("1231\n")),
            Ok((CompleteStr("\n"), 1231))
        );
    }

    #[test]
    fn test_parse_claim() {
        assert_eq!(
            parse_claim("#1 @ 817,273: 26x26"),
            Ok(Claim {
                number: 1,
                left: 817,
                top: 273,
                width: 26,
                height: 26,
            })
        );

        assert_eq!(
            parse_claim("#1 @ 817,273: 26x26\nSome garbage after"),
            Ok(Claim {
                number: 1,
                left: 817,
                top: 273,
                width: 26,
                height: 26,
            })
        );
    }
}
