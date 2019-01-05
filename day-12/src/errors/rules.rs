use std::{error::Error, fmt};

#[derive(Debug)]
pub(crate) enum RuleParseError {
    WrongRuleFormat(String),
    WrongPlantInfo(char),
}

impl fmt::Display for RuleParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuleParseError::WrongRuleFormat(input) => write!(f, "wrong rule format: {}", input),
            RuleParseError::WrongPlantInfo(c) => write!(f, "invalid plant state data: {}", c),
        }
    }
}

impl From<char> for RuleParseError {
    fn from(c: char) -> Self {
        RuleParseError::WrongPlantInfo(c)
    }
}

impl From<&str> for RuleParseError {
    fn from(s: &str) -> Self {
        RuleParseError::WrongRuleFormat(s.to_owned())
    }
}

impl Error for RuleParseError {}
