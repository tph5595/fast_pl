mod birthdeath;
use crate::birthdeath::BirthDeath;

mod barcode;

use clap::Parser;
use std::fs;

/// Generates the PL for a set of birth death pairs
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to read birth death pairs from
    #[clap(short, long, value_parser)]
    name: String,
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

    let filtered_pairs = barcode::barcode_filter(bd_pairs, 1);

    for bd in filtered_pairs {
        println!("{:?}", bd);
    }
}
