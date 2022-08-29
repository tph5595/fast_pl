use barcode;
use clap::Parser;
use std::fs;
use std::str::FromStr;

/// Generates the PL for a set of birth death pairs
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to read birth death pairs from
    #[clap(short, long, value_parser)]
    name: String,
}

#[derive(Debug)]
struct BirthDeath {
    birth: f32,
    death: f32,
}

impl FromStr for BirthDeath {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (b, d) = s.split_once(",").unwrap();

        return Ok(BirthDeath {
            birth: b.parse().unwrap(),
            death: d.parse().unwrap(),
        });
    }
}

fn main() {
    let args = Args::parse();

    let bd_pairs: Vec<BirthDeath> = fs::read_to_string(args.name)
        .unwrap()
        .trim()
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    for bd in bd_pairs {
        println!("{:?}", bd);
    }
}
