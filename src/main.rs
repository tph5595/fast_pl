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
    fn test_runner(k: usize, bd_pairs_vec: Vec<(f64, f64)>, answer_vec: &[Vec<(f64, f64)>]) {
        let bd_pairs = bd_pairs_vec
            .into_iter()
            .map(|(x, y)| fast_pl::birthdeath::BirthDeath { birth: x, death: y })
            .collect();
        // let answer: Vec<Vec<(f64,f64)>> = answer_vec
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
                (0.014296262174615215,0.0),
                (0.20041089668686668,0.18611463451225146),
                (0.2128825557428478,0.17364297545627033),
                (0.24232291853372195,0.20308333824714447),
                (0.28156217087526203,0.1638440859056044),
                (0.30197279957686435,0.18425471460720672),
                (0.48622751418407106,0.0),
            ],
            vec![
                (0.01673025014984808,0.0),
                (0.20142992797918263,0.18469967782933455),
                (0.21268459304754733,0.17344501276096985),
                (0.2128825557428478,0.17364297545627033),
                (0.2521218080843879,0.13440372311473026),
                (0.28156217087526203,0.1638440859056044),
                (0.4454062567808664,0.0),
            ],
            vec![
                (0.03923958028657748,0.0),
                (0.21268459304754733,0.17344501276096985),
                (0.2519238453890874,0.13420576041942978),
                (0.2521218080843879,0.13440372311473026),
                (0.38652553119911814,0.0),
            ],
            vec![
                (0.11771808496965763,0.0),
                (0.2519238453890874,0.13420576041942978),
                (0.3861296058085172,0.0),
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
                (0.13924980990143387,0.0),
                (0.4735505362112665,0.33430072630983265),
                (0.6730216860922412,0.1348295764288579),
                (0.6730217221747026,0.13482961251131925),
                (0.6882105426461086,0.11964079203991326),
                (0.7334269152420493,0.16485716463585398),
                (0.8982840798779033,0.0),
            ],
            vec![
                (0.5381921096633834,0.0),
                (0.6730216860922412,0.1348295764288579),
                (0.6882105065636472,0.11964075595745194),
                (0.6882105426461086,0.11964079203991326),
                (0.8078513346860219,0.0),
            ],
            vec![
                (0.5685697506061953,0.0),
                (0.6882105065636472,0.11964075595745194),
                (0.8078512625210992,0.0),
            ],
            ];
            test_runner(k, bd_pairs_vec, &answer_vec);
    }
    #[test]
    fn same_start() {
        let k = 2;
        let bd_pairs_vec = vec![
            (0.0, 1.0),
            (0.0, 2.0),
        ];
        let answer_vec = vec![
            vec![
                (0.0, 0.0),
                (0.5, 0.5),
                (1.0, 1.0),
                (2.0, 0.0),
            ],
            vec![
                (0.0, 0.0),
                (0.5, 0.5),
                (1.0, 0.0),
            ],
        ];
            test_runner(k, bd_pairs_vec, &answer_vec);
    }
    #[test]
    fn same_end() {
        let k = 2;
        let bd_pairs_vec = vec![
            (0.0, 3.0),
            (1.0, 3.0),
        ];
        let answer_vec = vec![
            vec![
                (0.0, 0.0),
                (1.5, 1.5),
                (2.0, 1.0),
                (3.0, 0.0),
            ],
            vec![
                (1.0, 0.0),
                (2.0, 1.0),
                (3.0, 0.0),
            ],
        ];
            test_runner(k, bd_pairs_vec, &answer_vec);
    }
    #[test]
    fn random_test() {
        let k = 4;
        let bd_pairs_vec = vec![
            (1.0, 8.0),
            (3.0, 7.0),
            (4.0, 9.0),
            (4.2, 10.0),
        ];
        let answer_vec = vec![
            vec![
                (1.0,0.0),
                (4.5,3.5),
                (6.0,2.0),
                (6.5,2.5),
                (6.6,2.4),
                (7.1,2.9),
                (10.0,0.0),
            ],
            vec![
                (3.0,0.0),
                (5.0,2.0),
                (5.5,1.5),
                (6.0,2.0),
                (6.1,1.9),
                (6.6,2.4),
                (9.0,0.0),
            ],
            vec![
                (4.0,0.0),
                (5.5,1.5),
                (5.6,1.4),
                (6.1,1.9),
                (8.0,0.0),
            ],
            vec![
                (4.2,0.0),
                (5.6,1.4),
                (7.0,0.0),
            ],
        ];
            test_runner(k, bd_pairs_vec, &answer_vec);
    }
    #[test]
    fn float_epsilon_intersection_landscape() {
        fn type_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let k = 2;
        let bd_pairs_vec = vec![
            (0.311_957_78, 0.469_108_1),
            (0.161_545_02, 0.311_957_977),
        ];
        let answer_vec = vec![
            vec![
                (0.16154502,0.0),
                (0.2367514985,0.07520647849999999),
                (0.3119578785,0.00000009849999998978731),
                (0.39053294,0.07857515999999998),
                (0.4691081,0.0),
            ],
            vec![
                (0.31195778,0.0),
                (0.3119578785,0.00000009849999998978731),
                (0.311957977,0.0)
            ],
        ];
            assert!("f64" == type_of(bd_pairs_vec[0].0), 
                "This test only works with f64. create new test for condition if type changes");
            test_runner(k, bd_pairs_vec, &answer_vec);
    }

}
