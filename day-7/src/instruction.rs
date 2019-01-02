use std::str::FromStr;

use nom::{anychar, call, delimited, map, named, separated_pair, tag, tuple_parser};

#[derive(Debug, PartialEq)]
pub(crate) struct OperationOrder {
    pub first: char,
    pub second: char,
}

// Step C must be finished before step A can begin.
named!(parse_instruction<&str, OperationOrder>,
    map!(
        delimited!(
            tag!("Step "),
            separated_pair!(
                anychar,
                tag!(" must be finished before step "),
                anychar
            ),
            tag!(" can begin.")
        ),
        |(first, second)| OperationOrder { first, second }
    )
);

#[derive(Debug, PartialEq)]
pub(crate) enum OperationOrderParseError {
    InvalidInput(String),
}

impl FromStr for OperationOrder {
    type Err = OperationOrderParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_instruction(s)
            .map(|(_, parsed)| parsed)
            .map_err(|_| OperationOrderParseError::InvalidInput(s.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            "Step C must be finished before step A can begin."
                .parse::<OperationOrder>()
                .unwrap(),
            OperationOrder {
                first: 'C',
                second: 'A',
            }
        );

        assert_eq!(
            "Step CDF must be finished before step A can begin.".parse::<OperationOrder>(),
            Err(OperationOrderParseError::InvalidInput(
                "Step CDF must be finished before step A can begin.".to_owned()
            )),
        );
    }
}
