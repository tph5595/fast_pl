mod barcode;
mod birthdeath;
mod persistencelandscape;
mod plot;

use clap::Parser;
use csv::Writer;
use std::error::Error;
use std::fs;

/// Generates the PL for a set of birth death pairs
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to read birth death pairs from
    #[clap(short, long, value_parser)]
    name: String,
    /// Max kth-landscape to calculate
    #[clap(short, long, value_parser, default_value_t = 1)]
    k: usize,
    /// Height of output image
    #[clap(short, long, value_parser, default_value_t = 720)]
    height: u32,
    /// Width of output image
    #[clap(short, long, value_parser, default_value_t = 1280)]
    width: u32,
    /// Enable debug messages
    #[clap(short, long, value_parser)]
    debug: bool,
    /// Save output image
    #[clap(short, long, value_parser)]
    graph: bool,
    /// Save to CSV
    #[clap(short, long, value_parser, default_value = "output.csv")]
    csv: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let bd_pairs: Vec<birthdeath::BirthDeath> = fs::read_to_string(args.name)
        .expect("File not found")
        .trim()
        .lines()
        .filter(|s| !s.contains("inf") && !s.is_empty())
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    if bd_pairs.len() == 0 {
        if args.debug {
            println!("No BirthDeath pairs found in file");
        }
        return Ok(());
    }

    if args.debug {
        println!("{:?}", bd_pairs);
    }
    let filtered_pairs = barcode::barcode_filter(bd_pairs, args.k);
    if args.debug {
        println!("{:?}", filtered_pairs);
    }
    let landscape = persistencelandscape::generate(filtered_pairs, args.k, args.debug);
    if args.debug {
        println!("{:?}", landscape);
    }
    let mut wtr = Writer::from_path(args.csv)?;
    for landscape in &landscape {
        for point in landscape {
            wtr.write_record(&[point.x.0.to_string(), point.y.0.to_string()])?;
        }
        wtr.write_record(&["", ""])?;
    }
    wtr.flush()?;
    return match args.graph {
        true => plot::plot_landscape(landscape, args.height, args.width),
        false => Ok(()),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_runner(k: usize, bd_pairs_vec: Vec<(f32, f32)>, answer_vec: Vec<Vec<(f32, f32)>>) {
        let bd_pairs = bd_pairs_vec
            .into_iter()
            .map(|(x, y)| birthdeath::BirthDeath { birth: x, death: y })
            .collect();
        let answer: Vec<Vec<persistencelandscape::PointOrd>> = answer_vec
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
        let landscape = persistencelandscape::generate(filtered_pairs, k, false);
        assert!(answer == landscape);
    }

    #[test]
    fn basic_triple() {
        let k = 4;
        let bd_pairs_vec = vec![(0.0, 6.0), (1.0, 3.0), (2.0, 7.0)];
        let answer_vec = vec![
            vec![(0.0, 0.0), (3.0, 3.0), (4.0, 2.0), (4.5, 2.5), (7.0, 0.0)],
            vec![(1.0, 0.0), (2.0, 1.0), (2.5, 0.5), (4.0, 2.0), (6.0, 0.0)],
            vec![(2.0, 0.0), (2.5, 0.5), (3.0, 0.0)],
            vec![],
        ];
        test_runner(k, bd_pairs_vec, answer_vec);
    }
    #[test]
    fn head_to_tail() {
        let k = 2;
        let bd_pairs_vec = vec![(1.0, 3.0), (3.0, 5.0)];
        let answer_vec = vec![
            vec![
                (1.0, 0.0),
                (2.0, 1.0),
                (3.0, 0.0),
                (3.0, 0.0),
                (4.0, 1.0),
                (5.0, 0.0),
            ],
            vec![],
        ];
        test_runner(k, bd_pairs_vec, answer_vec);
    }

    fn problem_pair_1() {
        let k = 4;
        let bd_pairs_vec = vec![
            (0.9748720526695251, 0.9898090958595276),
            (0.9600228071212769, 1.029630184173584),
            (0.8873197436332703, 0.9408737421035767),
        ];
        let answer_vec = vec![
            vec![(0.0, 0.0), (3.0, 3.0), (4.0, 2.0), (4.5, 2.5), (7.0, 0.0)],
            vec![(1.0, 0.0), (2.0, 1.0), (2.5, 0.5), (4.0, 2.0), (6.0, 0.0)],
            vec![(2.0, 0.0), (2.5, 0.5), (3.0, 0.0)],
            vec![],
        ];
        test_runner(k, bd_pairs_vec, answer_vec);
    }
}
