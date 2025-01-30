#![warn(
     clippy::all,
     clippy::pedantic,
     clippy::nursery,
     clippy::cargo,
 )]

use clap::Parser;
use csv::Writer;
use std::error::Error;
use std::fs;
use std::time::Instant;

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
    #[clap(short, long, value_parser, default_value = "")]
    csv: String,
    /// Disables filtering
    #[clap(short, long, value_parser)]
    disable_filter: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let now = Instant::now();
    let bd_paris: Vec<fast_pl::birthdeath::BirthDeath> = fs::read_to_string(args.name)?
        .lines()
        .filter(|s| !s.contains("inf") && !s.is_empty())
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let landscapes = fast_pl::rpls::pairs_to_landscape(bd_paris, args.k, args.debug, args.disable_filter)?;

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.?}");

    if !args.csv.is_empty() {
        let mut wtr = Writer::from_path(args.csv)?;
        for landscape in &landscapes {
            for point in landscape {
                wtr.write_record(&[point.0.to_string(), point.1.to_string()])?;
            }
            wtr.write_record(["", ""])?;
        }
        wtr.flush()?;
    }
    #[cfg(feature = "plot")]
    if args.graph {
        return fast_pl::plot::landscape(landscapes, args.height, args.width);
    }
    if args.debug{
        println!("Area: {}", fast_pl::rpls::l2_norm(&landscapes));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    fn test_runner(k: usize, bd_pairs_vec: Vec<(f32, f32)>, answer_vec: &[Vec<(f32, f32)>]) {
        let bd_pairs = bd_pairs_vec
            .into_iter()
            .map(|(x, y)| fast_pl::birthdeath::BirthDeath { birth: x, death: y })
            .collect();
        // let answer: Vec<Vec<(f32,f32)>> = answer_vec
        //     .into_iter()
        //     .map(|x| {
        //         x.into_iter()
        //             .map(|(x, y)| fast_pl::persistencelandscape::PointOrd {
        //                 x: float_ord::FloatOrd(x),
        //                 y: float_ord::FloatOrd(y),
        //             })
        //             .collect()
        //     })
        //     .collect();

        let filtered_pairs = fast_pl::barcode::filter(bd_pairs, k);
        let landscape = fast_pl::persistencelandscape::generate(filtered_pairs, k, false);
        assert!(answer_vec == landscape);
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
            test_runner(k, bd_pairs_vec, &answer_vec);
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
                (4.0, 1.0),
                (5.0, 0.0),
            ],
            vec![],
        ];
            test_runner(k, bd_pairs_vec, &answer_vec);
    }
    #[test]
    fn uniform_100_29() {
        let k = 4;
        let bd_pairs_vec = vec![
            (0.03923958028657748, 0.4454062567808664),
            (0.014296262174615215, 0.38652553119911814),
            (0.11771808496965763, 0.48622751418407106),
            (0.01673025014984808, 0.3861296058085172)
        ];
        let answer_vec = vec![
            vec![
                (0.0142962625,0.0),
                (0.2004109,0.18611464),
                (0.21288256,0.173643),
                (0.24232292,0.20308334),
                (0.28156218,0.16384408),
                (0.3019728,0.1842547),
                (0.4862275,0.0),
            ],
            vec![
                (0.01673025,0.0),
                (0.20142993,0.18469968),
                (0.2126846,0.17344502),
                (0.21288256,0.173643),
                (0.2521218,0.13440372),
                (0.28156218,0.16384408),
                (0.44540626,0.0),
            ],
            vec![
                (0.03923958,0.0),
                (0.2126846,0.17344502),
                (0.25192386,0.13420576),
                (0.2521218,0.13440372),
                (0.38652554,0.0),
            ],
            vec![
                (0.117718086,0.0),
                (0.25192386,0.13420576),
                (0.38612962,0.0),
            ],
            ];
            test_runner(k, bd_pairs_vec, &answer_vec);
    }
    #[test]
    fn uniform_2000_12() {
        let k = 3;
        let bd_pairs_vec = vec![
            (0.5381921096633834, 0.8078513346860219),
            (0.5685697506061953, 0.8982840798779033),
            (0.13924980990143387, 0.8078512625210992)
        ];
        let answer_vec = vec![
            vec![
                (0.13924982,0.0),
                (0.47355056,0.33430073),
                (0.6730217,0.13482961),
                // (0.6730217,0.13482961),
                (0.68821055,0.11964076),
                (0.7334269,0.16485715),
                (0.8982841,0.0),
            ],
            vec![
                (0.5381921,0.0),
                (0.6730217,0.13482961),
                (0.68821055,0.11964075),
                (0.8078513,0.0),
            ],
            vec![
                (0.5685698,0.0),
                (0.68821055,0.11964075),
                (0.80785125,0.0),
            ],
            ];
            test_runner(k, bd_pairs_vec, &answer_vec);
    }
}
