#![warn(
     clippy::all,
     clippy::pedantic,
     clippy::nursery,
     clippy::cargo,
 )]

use std::str::FromStr;

#[derive(Debug)]
pub struct BirthDeath {
    pub birth: f64,
    pub death: f64,
}

impl FromStr for BirthDeath {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (b, d) = s.split_once(' ').unwrap();

        return Ok(Self {
            birth: b.trim().parse().unwrap(),
            death: d.trim().parse().unwrap(),
        });
    }
}
