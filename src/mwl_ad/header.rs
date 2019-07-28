use std::str;
use nom::{ IResult };
use nom;
use nom::bytes::complete as noms;
use nom::sequence::{delimited};
use nom::multi::{separated_list};
use nom::branch;
use nom::combinator;
use nom::sequence;

// TODO: I want to specify that this is the _real_ public API,
// and that everything below `require` is only public in case
// someone downstream really needs it.
//
// How do you format documentation in rust?

/// Errors related to header file parsing and key lookup
#[derive (Clone, Debug, PartialEq)]
pub enum HeaderError {
    UnknownKey { key: String },
    ParseError { err: String },
}

/// Abstract metadata collection derived from file header
pub struct Metadata <'a> {
    pub header: Vec<HeaderLine<'a>>
}

/// Parse Metadata and get a pointer to the file's binary data
pub fn parse(file_contents: &str) -> Result<(Metadata, &str), HeaderError> {
    match parse_header(file_contents) {
        Ok ((file_data, lines)) =>
            Ok ((Metadata { header: lines }, file_data)),
        Err (e) =>
            Err (HeaderError::ParseError {
                err: format!("header parse error: {:?}", e)
            }),
    }
}

/// Find the value for the first occurrance of `key` in the metadata
pub fn lookup<'a, 'b>(metadata: &Metadata<'a>, key: &'b str) -> Option<&'a str> {
    metadata
        .header
        .iter()
        .filter_map(|hl| match hl {
            HeaderLine::HeaderPair { key: k , value: v }
            if k == &key => Some (v.to_owned()),
            _ => None,
        })
        .next()
}


/// Find the value of the first occurrance of `key` in metadata,
/// returning an error if `key` is missing
pub fn require<'a, 'b>(metadata: &Metadata<'a>, key: &'b str) -> Result<&'a str, HeaderError> {
    lookup(metadata, key)
        .map(|v| Ok(v))
        .unwrap_or( Err (HeaderError::UnknownKey { key: key.to_owned() }) )
}


/// Find all values for `key` in metadata
pub fn lookup_multiple<'a, 'b>(metadata: Metadata<'a>, key: &'b str) -> Vec<&'a str> {
    metadata
        .header
        .into_iter()
        .filter_map(|hl| match hl {
            HeaderLine::HeaderPair { key: k, value: v} if k == key => Some (v),
            _ => None
        })
        .collect()
}


pub fn parse_header(s : &str) -> IResult<&str, Vec<HeaderLine>> {
    delimited( noms::tag("%%BEGINHEADER\n"),
               separated_list( noms::tag("\n"), header_line ),
               noms::tag("\n%%ENDHEADER\n")
    )(s)
}


#[derive (Clone, Copy, Debug, PartialEq)]
pub enum HeaderLine <'a> {
    HeaderPair { key: & 'a str, value: & 'a str },
    HeaderComment { comment: & 'a str },
}


fn header_line(line: &str) -> IResult<&str, HeaderLine> {
    dbg!(branch::alt(
        (sequence::preceded( noms::tag("% "), header_pair ) ,
         header_comment)
    )(line))
    // sequence::preceded(
    //     noms::tag("% "),
    //     branch::alt( (header_pair, header_comment) )
    // )(line)
}

// Parses like this:
// "key : value" -> HeaderPair { key: "key", value: "value" }
fn header_pair(line: &str) -> IResult<&str, HeaderLine> {
    combinator::map(
        sequence::separated_pair(
            noms::take_while1(|ch| ch != ':' && ch != '\n'),
            noms::tag(":"),
            noms::take_while1(|ch| ch != '\n')
        ),
        |(k,v) : (&str, &str)| HeaderLine::HeaderPair {
            key: k.trim(),
            value: v.trim()
        }
    )(line)
}

// Parses like this:
// "% Any: thing" -> HeaderComment {comment: "Any: thing"}
// "%\n"          -> HeaderComment {comment: ""}
// The ':' signifying Pair (as opposed to Comment) is handled
// upstream of this parser, so we don't need to handle it here
fn header_comment(line: &str) -> IResult<&str, HeaderLine> {
    branch::alt(
        (combinator::map(
            sequence::preceded(noms::tag("% "), noms::take_while(|ch| ch != '\n')),
            |s| HeaderLine::HeaderComment { comment: s }),
         combinator::value(
             HeaderLine::HeaderComment { comment: "" },
             sequence::preceded(
                 noms::tag("%"),
                 combinator::peek( noms::tag("\n") )
             )
         )
        )
    )(line)
}
                                  

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_header_pair() {
        assert_eq!(header_pair("key : value"),
                   Ok(("", HeaderLine::HeaderPair { key: "key",
                                                    value: "value" })
                   )
        );
    }
    #[test]
    fn it_parses_header_comment() {
        assert_eq!(header_comment("% Some comment"),
                   Ok(("", HeaderLine::HeaderComment { comment: "Some comment" }))
        );
    }

    #[test]
    fn it_stops_after_header_pair() {
        assert_eq!(header_line("% key: value\nleftover"),
                   Ok(("\nleftover", HeaderLine::HeaderPair{key: "key", value: "value"}))
        );
    }

    #[test]
    fn it_stops_after_header_comment() { 
        assert_eq!(header_line("% Some comment\nleftover"),
                   Ok(("\nleftover", HeaderLine::HeaderComment{comment: "Some comment"}))
        );
    }

    #[test]
    fn it_parses_header_line_small () {
        let r = parse_header(HEADER_FIXTURE_SMALL);
        assert_eq!(
            r.clone().map(|vs| vs.1[0]),
            Ok (HeaderLine::HeaderPair { key: "Program", value: "./adextract"})
        );
        assert_eq!(
            r.map(|vs| vs.1[1]),
            Ok (HeaderLine::HeaderComment { comment: "Some comment" })
        );
    }

    #[test]
    fn it_parses_header_big () {
        let (m,_) = parse(HEADER_FIXTURE).unwrap();
        assert_eq!(lookup(&m, "Program"), Some("./adextract"));
        assert_eq!(require(&m, "Argc"), Ok("8"));
    }
}

const HEADER_FIXTURE_SMALL : &'static str = r#"%%BEGINHEADER
% Program: 	./adextract
% Some comment
%
% Program Version: 	1.18
% Argc: 	8
%%ENDHEADER
"#;
    
const HEADER_FIXTURE : &'static str = r#"%%BEGINHEADER
% Program: 	./adextract
% Program Version: 	1.18
% Argc: 	8
% Argv[1] :	data/original.spk.raw
% Argv[2] :	-eslen80
% Argv[3] :	-t
% Argv[4] :	-probe
% Argv[5] :	0
% Argv[6] :	-o
% Argv[7] :	data/original.tt
% Date: 	Mon Oct 22 16:49:04 2012
% Directory: 	/home/stuart/src/mwl-svn-export/bin
% Hostname: 	ubuntu
% Architecture: 	i686
% User: 	stuart ()
% File type: 	Binary
% Extraction type: 	tetrode waveforms
% Probe: 	0
% Fields: 	timestamp,8,4,1	waveform,2,2,128	
%
% Beginning of header from input file 'data/original.spk.raw'
% mode: SPIKE
% adversion: 		1.36b
% rate: 		250000.000000
% nelectrodes: 2
% nchannels: 		8
% nelect_chan: 		4
% errors: 		0
% disk_errors: 		0
% dma_bufsize: 		24576
% spikelen: 		32
% spikesep: 		26
% channel 0 ampgain: 	24994
% channel 0 adgain: 	0
% channel 0 filter: 	200
% channel 0 threshold: 	425
% channel 0 color: 		15
% channel 0 offset: 	65
% channel 0 contscale: 	0
% channel 1 ampgain: 	24994
% channel 1 adgain: 	0
% channel 1 filter: 	200
% channel 1 threshold: 	425
% channel 1 color: 		14
% channel 1 offset: 	122
% channel 1 contscale: 	0
% channel 2 ampgain: 	24994
% channel 2 adgain: 	0
% channel 2 filter: 	200
% channel 2 threshold: 	425
% channel 2 color: 		13
% channel 2 offset: 	180
% channel 2 contscale: 	0
% channel 3 ampgain: 	24994
% channel 3 adgain: 	0
% channel 3 filter: 	200
% channel 3 threshold: 	425
% channel 3 color: 		12
% channel 3 offset: 	238
% channel 3 contscale: 	0
% channel 4 ampgain: 	24994
% channel 4 adgain: 	0
% channel 4 filter: 	200
% channel 4 threshold: 	325
% channel 4 color: 		11
% channel 4 offset: 	296
% channel 4 contscale: 	0
% channel 5 ampgain: 	24994
% channel 5 adgain: 	0
% channel 5 filter: 	200
% channel 5 threshold: 	325
% channel 5 color: 		10
% channel 5 offset: 	353
% channel 5 contscale: 	0
% channel 6 ampgain: 	24994
% channel 6 adgain: 	0
% channel 6 filter: 	200
% channel 6 threshold: 	325
% channel 6 color: 		9
% channel 6 offset: 	411
% channel 6 contscale: 	0
% channel 7 ampgain: 	24994
% channel 7 adgain: 	0
% channel 7 filter: 	200
% channel 7 threshold: 	325
% channel 7 color: 		8
% channel 7 offset: 	469
% channel 7 contscale: 	0
% spike_size: 		264
% fields: int electrode_num; long timestamp; int data[128]
% End of header from input file 'data/original.spk.raw'
%
%%ENDHEADER
"#;
