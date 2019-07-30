use std::fs::{File};
use std::io::Read;
use std::str;

use nom::{IResult};
use nom::combinator as nomc;
use nom::sequence as noms;
use nom::multi as nomm;
use nom::number::complete as nomnum;

// use nom::combinator as nom;

use super::{Spike, SpikeSI};
// use super::mwl_ad;
use crate::mwl_ad::header;



#[derive (Debug)]
struct Metadata {
}

pub fn read_spikes(file_path: &str) -> Vec<Spike<f32,f32>> {
    let mut buffer = Vec::new();
    File::open(file_path).unwrap().read_to_end(&mut buffer).unwrap();
    let (header, file_binary) = header::parse(buffer.as_slice()).unwrap();
    let probe_ind_0 = match header::require( &header, "Probe" ) {
        Ok("0") => 0,
        Ok("1") => 4,
        Ok(s)   => panic!("Unknown probe: {}. It should be \"0\" or \"1\"", s ),
        Err(s)  => panic!("error reading probe index: {:?}", s),
    };

    let gain = |n: i8| {
        let key = format!("channel {} ampgain", n);
        let gain_str : &str = header::require(&header, key.as_str()).unwrap();
        gain_str.parse::<f32>().unwrap()
    };
    let gains = vec![gain(probe_ind_0),
                     gain(probe_ind_0 + 1),
                     gain(probe_ind_0 + 2),
                     gain(probe_ind_0 + 3)];

    let spikes = parse_spikes( gains, file_binary );
    spikes.unwrap().1
}



fn parse_spikes(gains : Vec<f32>, input: &[u8]) -> IResult<&[u8], Vec<Spike<f32,f32>>> {
    nomm::many0(
    nomc::map(
        noms::pair(
            nomnum::le_i32, // unsigned long
            nomm::count(nomnum::le_i16, 128)
        ),
        |(t,vs)| {
            let time = t as f32 / 10_000.0;
            // let scale_v = move |v : &i16| (*v as f32);
            let voltages : Vec<f32> = vs
                .into_iter()
                .zip( gains.iter().cycle() )
                .map(|(v,g)| (v as f32) / 32_768.0 * 5.0 / g )
                .collect();

            let mut waveforms : Vec<Vec<f32>> = Vec::new();
            for _ in 0..4 {
                waveforms.push (Vec::new());
            }
            for i in 0..voltages.len() {
                let chan = i % 4;
                waveforms[chan].push(voltages[i]);
            };
            // let waveforms : Vec<Vec<f32>> = vec![to_waveform((0,31)),
            //                                      to_waveform((32,63)),
            //                                      to_waveform((64,95)),
            //                                      to_waveform((96,127))
            // ];
            Spike {time, waveforms}
        }
    ))(input)
}


pub fn placeholder() {
    let my_test_spike: Spike<u32, u32> = Spike {
        waveforms: vec![vec![0, 1, 2]],
        time: 1,
    };
    println!("hi. it's {:?}", my_test_spike);
}
