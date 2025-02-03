#![warn(
     clippy::all,
     clippy::pedantic,
     clippy::nursery,
     clippy::cargo,
 )]

use std::cmp;
use float_ord::FloatOrd;

use crate::birthdeath::BirthDeath;
use crate::persistencelandscape;
use crate::barcode;

/// # Errors
///
/// Will return 'Err' if failed to compute persistencelandscape from `bd_pairs`
pub fn pairs_to_landscape(bd_pairs: Vec<BirthDeath>, k:usize, debug:bool, disable_filter: bool) -> Result<Vec<Vec<(f64,f64)>>, &'static str>{
    let bd_pairs: Vec<BirthDeath> = bd_pairs
        .into_iter()
        .filter(|bd| (bd.birth - bd.death).abs() > f64::EPSILON)
        .collect();
    if bd_pairs.is_empty() {
        return Err("No BirthDeath pairs found in file");
    }

    if debug {
        println!("{bd_pairs:?}");
    }
    let filtered_pairs = if disable_filter{
        bd_pairs
    }
    else{
        let filtered_pairs = barcode::filter(bd_pairs, k);
        if debug {
            println!("{filtered_pairs:?}");
        }
        filtered_pairs
    };
    let landscape = persistencelandscape::generate(filtered_pairs, k, debug);
    if debug {
        println!("{landscape:?}");
    }
    Ok(landscape)
}

fn area_under_line_segment(a: (f64,f64), b: (f64,f64)) ->f64 {
    let height = (a.1 - b.1).abs();
    let base = a.0 - b.0;
    let triangle = (height * base) / 2.0;
    assert!(triangle > 0.0);

    let s1 = base;
    let s2 = cmp::min(FloatOrd(a.1), FloatOrd(b.1)).0;
    let rectangle = s1 * s2;
    assert!(rectangle >= 0.0);

    triangle + rectangle
}

fn landscape_norm(landscape: &[(f64,f64)]) -> f64 {
    landscape
        .iter()
        .zip(landscape.iter().skip(1))
        .map(|(a, b)| area_under_line_segment(*a, *b))
        .sum::<f64>()
}

fn is_sorted<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}

/// # Panics
///
/// Will panic if areas are not strictly decreasing or equal
#[must_use]
pub fn l2_norm(landscapes: &[Vec<(f64,f64)>]) -> f64 {
    let areas = landscapes
        .iter()
        .map(|l| FloatOrd(landscape_norm(l)))
        .collect::<Vec<FloatOrd<f64>>>();
    assert!(is_sorted(& areas));

        areas.iter().map(|x| x.0).sum()
}

/// # Errors
///
/// Will return 'Err' if failed to compute persistencelandscape from `bd_pairs`
pub fn pairs_to_l2_norm(bd_paris: Vec<BirthDeath>, k:usize, debug:bool, disable_filter: bool) -> Result<f64, &'static str>{
    Ok(l2_norm(pairs_to_landscape(bd_paris, k, debug, disable_filter)?.as_slice()))
}
