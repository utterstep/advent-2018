use std::{error::Error, fmt, num::ParseIntError, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Position {
    pub(super) x: i64,
    pub(super) y: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Velocity {
    x_vel: i64,
    y_vel: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Star {
    pub(super) position: Position,
    pub(super) velocity: Velocity,
}

impl Star {
    pub fn advance(&mut self) {
        self.position.x += self.velocity.x_vel;
        self.position.y += self.velocity.y_vel;
    }

    pub fn rewind(&mut self) {
        self.position.x -= self.velocity.x_vel;
        self.position.y -= self.velocity.y_vel;
    }
}

#[derive(Debug)]
pub(crate) enum StarParseError {
    WrongFormat(String),
    ParseError(ParseIntError),
}

impl fmt::Display for StarParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_string = match self {
            StarParseError::WrongFormat(s) => format!("Wrong incoming string: {}", s),
            StarParseError::ParseError(e) => format!("{}", e),
        };

        write!(f, "{}", error_string)
    }
}

impl Error for StarParseError {}

// position=< -9945,  20347> velocity=< 1, -2>

impl FromStr for Star {
    type Err = StarParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref STAR_RE: Regex = Regex::new(
                r"(?x)
                position=<
                    \s*
                    (?P<x>\-?\d+),
                    \s*
                    (?P<y>\-?\d+)
                >\s+
                velocity=<
                    \s*
                    (?P<x_vel>\-?\d+),
                    \s*
                    (?P<y_vel>\-?\d+)>
                "
            )
            .unwrap();
        }

        let matches = STAR_RE
            .captures(s)
            .ok_or_else(|| StarParseError::WrongFormat(s.to_owned()))?;

        let x = matches["x"].parse().map_err(StarParseError::ParseError)?;
        let y = matches["y"].parse().map_err(StarParseError::ParseError)?;
        let x_vel = matches["x_vel"]
            .parse()
            .map_err(StarParseError::ParseError)?;
        let y_vel = matches["y_vel"]
            .parse()
            .map_err(StarParseError::ParseError)?;

        Ok(Star {
            position: Position { x, y },
            velocity: Velocity { x_vel, y_vel },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_advance() {
        let mut star = Star {
            position: Position { x: 0, y: 0 },
            velocity: Velocity { x_vel: 1, y_vel: 2 },
        };

        star.advance();
        assert_eq!(star.position, Position { x: 1, y: 2 });
        assert_eq!(star.velocity, Velocity { x_vel: 1, y_vel: 2 });

        star.advance();
        assert_eq!(star.position, Position { x: 2, y: 4 });
        assert_eq!(star.velocity, Velocity { x_vel: 1, y_vel: 2 });

        let mut star = Star {
            position: Position { x: 0, y: 0 },
            velocity: Velocity { x_vel: 0, y_vel: 3 },
        };

        star.advance();
        assert_eq!(star.position, Position { x: 0, y: 3 });
        assert_eq!(star.velocity, Velocity { x_vel: 0, y_vel: 3 });

        star.advance();
        assert_eq!(star.position, Position { x: 0, y: 6 });
        assert_eq!(star.velocity, Velocity { x_vel: 0, y_vel: 3 });

        let mut star = Star {
            position: Position { x: 5, y: 7 },
            velocity: Velocity {
                x_vel: -1,
                y_vel: -3,
            },
        };

        star.advance();
        assert_eq!(star.position, Position { x: 4, y: 4 });
        assert_eq!(
            star.velocity,
            Velocity {
                x_vel: -1,
                y_vel: -3,
            }
        );

        star.advance();
        assert_eq!(star.position, Position { x: 3, y: 1 });
        assert_eq!(
            star.velocity,
            Velocity {
                x_vel: -1,
                y_vel: -3,
            }
        );
    }

    #[test]
    fn test_star_rewind() {
        let mut star = Star {
            position: Position { x: 5, y: 7 },
            velocity: Velocity {
                x_vel: -1,
                y_vel: -3,
            },
        };

        star.advance();
        assert_eq!(star.position, Position { x: 4, y: 4 });
        assert_eq!(
            star.velocity,
            Velocity {
                x_vel: -1,
                y_vel: -3,
            }
        );

        star.advance();
        assert_eq!(star.position, Position { x: 3, y: 1 });
        assert_eq!(
            star.velocity,
            Velocity {
                x_vel: -1,
                y_vel: -3,
            }
        );

        star.rewind();
        assert_eq!(star.position, Position { x: 4, y: 4 });
        assert_eq!(
            star.velocity,
            Velocity {
                x_vel: -1,
                y_vel: -3,
            }
        );

        star.rewind();
        assert_eq!(star.position, Position { x: 5, y: 7 });
        assert_eq!(
            star.velocity,
            Velocity {
                x_vel: -1,
                y_vel: -3,
            }
        );
    }

    #[test]
    fn test_star_parse() {
        assert_eq!(
            "position=<0, 0> velocity=<1, 2>".parse::<Star>().unwrap(),
            Star {
                position: Position { x: 0, y: 0 },
                velocity: Velocity { x_vel: 1, y_vel: 2 },
            }
        );
        assert_eq!(
            "position=<-10, 0> velocity=<1, 25>"
                .parse::<Star>()
                .unwrap(),
            Star {
                position: Position { x: -10, y: 0 },
                velocity: Velocity {
                    x_vel: 1,
                    y_vel: 25
                },
            }
        );

        assert!("position=<-10, 0> velocity=<1, 25f>"
            .parse::<Star>()
            .is_err());
    }
}
