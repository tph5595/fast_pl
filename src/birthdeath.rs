use std::str::FromStr;

#[derive(Debug)]
pub struct BirthDeath {
    pub birth: f32,
    pub death: f32,
}

impl FromStr for BirthDeath {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (b, d) = s.split_once(" ").unwrap();

        return Ok(BirthDeath {
            birth: b.trim().parse().unwrap(),
            death: d.trim().parse().unwrap(),
        });
    }
}
