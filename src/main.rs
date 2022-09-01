mod barcode;
mod birthdeath;
mod persistencelandscape;
mod plot;

use clap::Parser;
use std::error::Error;
use std::fs;

/// Generates the PL for a set of birth death pairs
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to read birth death pairs from
    #[clap(short, long, value_parser)]
    name: String,
    #[clap(short, long, value_parser)]
    k: usize,
    #[clap(short, long, value_parser)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
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
    return plot::plot_landscape(landscape);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let k = 4;
        let bd_pairs = vec![(0.0, 6.0), (1.0, 3.0), (2.0, 7.0)]
            .into_iter()
            .map(|(x, y)| birthdeath::BirthDeath { birth: x, death: y })
            .collect();
        let answer: Vec<Vec<persistencelandscape::PointOrd>> = vec![
            vec![(0.0, 0.0), (3.0, 3.0), (4.0, 2.0), (4.5, 2.5), (7.0, 0.0)],
            vec![(1.0, 0.0), (2.0, 1.0), (2.5, 0.5), (4.0, 2.0), (6.0, 0.0)],
            vec![(2.0, 0.0), (2.5, 0.5), (3.0, 0.0)],
            vec![],
        ]
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|(x, y)| persistencelandscape::PointOrd {
                    x: float_ord::FloatOrd(x),
                    y: float_ord::FloatOrd(y),
                })
                .collect()
        })
        .collect();

        let filtered_pairs = barcode::barcode_filter(bd_pairs, k);
        let landscape = persistencelandscape::generate(filtered_pairs, k);
        assert!(answer == landscape);
    }
}
