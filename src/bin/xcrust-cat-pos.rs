use std::env;
use std::path::{Path};

use xcrust::pos::mwl_ad::{read_p};

fn main() {
    let args : Vec<String> = env::args().collect();
    let file = &args[1];
    let ps = read_p( Path::new(&file) );
    println!("ps[0]: {:?}", ps[0]);
    println!("ps[0]: {:?}", ps[0]);
}
