use crate::birthdeath::BirthDeath;
use crate::persistencelandscape;
use crate::barcode;

pub fn pairs_to_landscape(bd_paris: Vec<BirthDeath>, k:usize, debug:bool) -> Result<Vec<Vec<persistencelandscape::PointOrd>>, &'static str>{
    let bd_pairs: Vec<BirthDeath> = bd_paris
        .into_iter()
        .filter(|bd| ! (bd.birth == bd.death))
        .collect();
    if bd_pairs.len() == 0 {
        if debug {
            println!("No BirthDeath pairs found in file");
        }
        return Ok(persistencelandscape::empty_landscape(k));
    }

    if debug {
        println!("{:?}", bd_pairs);
    }
    let filtered_pairs = barcode::barcode_filter(bd_pairs, k);
    if debug {
        println!("{:?}", filtered_pairs);
    }
    let landscape = persistencelandscape::generate(filtered_pairs, k, debug);
    if debug {
        println!("{:?}", landscape);
    }
    return Ok(landscape);
}

fn area_under_line_segment(a: persistencelandscape::PointOrd, b: persistencelandscape::PointOrd) ->f32 {
    let height = (a.y.0 - b.y.0).abs();
    let base = (a.x.0 - b.x.0).abs();
    (height * base) / 2.0
}

fn landscape_norm(landscape: &Vec<persistencelandscape::PointOrd>) -> f32 {
    landscape
        .iter()
        .zip(landscape.iter().skip(1))
        .map(|(&a, &b)| area_under_line_segment(a, b))
        .sum::<f32>()
}
pub fn l2_norm(landscapes: Vec<Vec<persistencelandscape::PointOrd>>) -> f32 {
    landscapes
        .iter()
        .map(|landscape| landscape_norm(landscape))
        .sum()
}

pub fn pairs_to_l2_norm(bd_paris: Vec<BirthDeath>, k:usize, debug:bool) -> f32{
    l2_norm(pairs_to_landscape(bd_paris, k, debug).unwrap())
}
