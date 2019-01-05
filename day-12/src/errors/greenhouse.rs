use std::{error::Error, fmt};

use super::rules::RuleParseError;

#[derive(Debug)]
pub(crate) enum GreenhouseParseError {
    IncompleteData,
    InvalidStateDefinition(String),
    InvalidRuleDefinition(RuleParseError),
}

impl fmt::Display for GreenhouseParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GreenhouseParseError::IncompleteData => write!(f, "insufficient data for parsing"),
            GreenhouseParseError::InvalidStateDefinition(state) => {
                write!(f, "invalid state definition: {}", state)
            }
            GreenhouseParseError::InvalidRuleDefinition(rule_error) => {
                write!(f, "got error while parsing the rule: {}", rule_error)
            }
        }
    }
}

impl From<&str> for GreenhouseParseError {
    fn from(s: &str) -> Self {
        GreenhouseParseError::InvalidStateDefinition(s.to_owned())
    }
}

impl From<RuleParseError> for GreenhouseParseError {
    fn from(error: RuleParseError) -> Self {
        GreenhouseParseError::InvalidRuleDefinition(error)
    }
}

impl Error for GreenhouseParseError {}
