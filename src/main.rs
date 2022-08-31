mod barcode;
mod birthdeath;
mod persistencelandscape;

use clap::Parser;
use std::fs;

/// Generates the PL for a set of birth death pairs
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to read birth death pairs from
    #[clap(short, long, value_parser)]
    name: String,
    #[clap(short, long, value_parser)]
    k: i32,
    #[clap(short, long, value_parser)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let bd_pairs: Vec<birthdeath::BirthDeath> = fs::read_to_string(args.name)
        .unwrap()
        .trim()
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    if args.debug {
        println!("{:?}", bd_pairs);
    }
    let filtered_pairs = barcode::barcode_filter(bd_pairs, args.k);
    if args.debug {
        println!("{:?}", filtered_pairs);
    }
    let landscape = persistencelandscape::generate(filtered_pairs, args.k);
    if args.debug {
        println!("{:?}", landscape);
    }
}
