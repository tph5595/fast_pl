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
