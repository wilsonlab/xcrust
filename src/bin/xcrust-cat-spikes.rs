use std::io::prelude::*;

use std::io::stdin;
use xcrust::spike::mwl_ad::{read_spikes};
use xcrust::spike::ascii_draw;
use chrono::Duration;

use clap::{crate_version, App, Arg, arg_enum, value_t};
use clap as Clap;

use std::collections::HashMap;
use std::path::{PathBuf};

/// Required config for the main conversion command
#[derive(Clone, Debug)]
struct ConvertConfig {
    input_file: PathBuf,
    input_format: InputFormat,
    output_format: OutputFormat,
    after: Option<Duration>,
    before: Option<Duration>,
}


arg_enum!{
/// We support only one input format for now: mwl ad .tt file
#[derive(Clone, Debug)]
enum InputFormat {
    Ad,
}
}

arg_enum!{
/// We support only one output format for now: the
/// debugging format of `Spike`
#[derive(Clone, Debug)]
enum OutputFormat {
    SpikeDebug,
    SpikeJSON,
    SpikeAscii,
}
}

fn main() {
    let config = parse_config().unwrap_or_else(|_| panic!("TODO"));
    // let mut b = [0; 10];
    // File::open(config.input_file).unwrap().read(&mut b).unwrap();
    // println!("config: {:?}", config);
    
    // println!("b: {:?}", read_spikes(config.input_file.as_path()));
    let spikes = read_spikes( config.input_file.to_str().unwrap() );
    match config.output_format {
        OutputFormat::SpikeDebug => println!("spikes: {:?}", spikes),
        OutputFormat::SpikeJSON => panic!("not implemented"),
        OutputFormat::SpikeAscii => for s in spikes {
            let plot = ascii_draw::DEFAULT_PLOT;
            ascii_draw::draw_spike(plot, &s.waveforms);
            let _ = stdin().read(&mut [0u8]).unwrap();
        },
        
    }
}



fn input_format(
    input_type: Option<InputFormat>,
    input_file_ext: Option<&str>,
) -> InputFormat {
    input_type.unwrap_or_else(
        || match input_file_ext {
            None => (Clap::Error::with_description(
                "Must specify an --input-format, or use a \
                 file with a known extension",
                Clap::ErrorKind::ValueValidation
            )).exit(),
            Some(input_ext) => {
                let mut extension_formats: HashMap<&str, InputFormat> =
                    HashMap::new();
                extension_formats.insert("tt", InputFormat::Ad);
                extension_formats
                    .get(input_ext)
                    .unwrap_or_else(|| Clap::Error::with_description(
                        format!("Extension {} is not in the list \
                                 of known extensions: {:?}. Please use \
                                 --input-format",
                                input_ext,
                                extension_formats.keys()
                        ).as_str(),
                        Clap::ErrorKind::ValueValidation).exit())
                    .clone()
            }
        })
}


fn parse_config() -> Result<ConvertConfig, String> {

    // Specify commandline interface
    let matches = App::new("xcrust-cat-spikes")
        .version(crate_version!())
        .arg(Arg::from_usage("<input-file> 'File to read spikes from'"))
        .arg(Arg::from_usage("-i, --input-format [input-format] \
                              'Input file format (leave unset to infer from FILE)'")
             .possible_values(&InputFormat::variants()))
        .arg(Arg::from_usage("-o, --output-format [output-format] \
                              'Output file format'")
             .possible_values(&OutputFormat::variants()))
        .arg(Arg::from_usage("-a --after [after] 'Lower bound on spikes to cat'"))
        .arg(Arg::from_usage("-b --before [before] 'Upper bound on spikes to cat'"))
        .get_matches();

    // Parse out the parts needed for inferring the input-format
    let input_file = value_t!(matches, "input-file", PathBuf).unwrap();
    let file_ext = input_file.extension().and_then(|p| p.to_str());
    let input_format_arg = map_well_formed(
        value_t!(matches, "input-format", InputFormat),
        |f| f
    );

    // Parse input and output formats
    let input_format = input_format(input_format_arg, file_ext);
    let output_format = value_t!(matches, "output-format", OutputFormat)
        .unwrap_or(OutputFormat::SpikeDebug);

    fn map_well_formed<F,A,B>(r: Result<A, Clap::Error>, f: F) -> Option<B>
    where
        F: Fn(A) -> B,
    {
        match r {
            Ok  (a) => Some(f(a)),
            Err (Clap::Error { kind: Clap::ErrorKind::ArgumentNotFound, .. } ) => None,
            Err (e) => e.exit(),
        }
    }

    fn get_duration(r : Result<f64, Clap::Error>) -> Option<Duration> {
        map_well_formed( r, |t| Duration::nanoseconds( (t * 1_000_000_000.0) as i64) )
    }
    
    let after = get_duration(value_t!(matches, "after", f64));
    let before = get_duration(value_t!(matches, "before", f64));

    Ok(ConvertConfig {
        input_file: input_file.to_path_buf(),
        input_format: input_format.clone(),
        output_format,
        after,
        before,
    })
}
