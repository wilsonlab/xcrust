use std::path::{Path};
use std::fs::{File};
use std::io::Read;
use nom::combinator as nomc;
use nom::sequence as noms;
use nom::multi as nomm;
use nom::number::complete as nomnum;
use nom::{IResult};

use crate::mwl_ad::header;
use super::{DiodePos};


pub fn read_p(path: &Path) -> Vec<DiodePos<f32, f32>> {
    let mut buffer = Vec::new();
    File::open(path).unwrap().read_to_end(&mut buffer).unwrap();
    let (_, file_binary) = header::parse(buffer.as_slice()).unwrap();
    parse_p( file_binary ).unwrap().1
}

pub fn parse_p(str: &[u8]) -> IResult< &[u8], Vec<DiodePos<f32, f32>> > {
    nomm::many0(
        nomc::map(
        noms::pair(
            nomnum::le_u32, //unsigned long (timestamp)
            nomm::count( nomnum::le_i16, 4)
        ),
        |(t,pos_coords)| DiodePos {
            time:        t as f32 / 10_000.0,
            diode_front: (pos_coords[0] as f32, pos_coords[1] as f32),
            diode_back:  (pos_coords[2] as f32, pos_coords[3] as f32),
        }
    ))(str)
}

