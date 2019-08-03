use std::path::{Path};

use xcrust::pos::mwl_ad::{read_p};

fn main() {
    let file = "../mwsoft64/tests/data/original.p";
    let ps = read_p( Path::new(file) );
    println!("ps[0]: {:?}", ps[0]);
    println!("ps[0]: {:?}", ps[0]);
}
