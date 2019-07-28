pub mod header;

use std::path::{Path};

#[derive(Debug, FromPrimitive, PartialEq, ToPrimitive)]
pub enum FormatType {
    InvalidT = 0,
    CharT    = 1,
    ShortT   = 2,
    IntT     = 3,
    FloatT   = 4,
    DoubleT  = 5,
    FuncT    = 6,
    FFuncT   = 7,
    ULongT   = 8,
    UnknownT = -1,
}

#[derive(Debug,Fail, PartialEq)]
pub enum DecodingError {
    #[fail(display = "invalid format code: {}", code)]
    UnknownFormatType { code : i32 },
}

pub fn decode_type(i: i32) -> Result<FormatType,DecodingError> {
    num::FromPrimitive::from_i32(i)
        .map(Ok)
        .unwrap_or(Err(DecodingError::UnknownFormatType {code: i}))
}

pub fn read_spikes(_fp: &Path) -> () {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_type_int() {
        assert_eq!(decode_type(3), Ok (FormatType::IntT));
        assert_eq!(decode_type(-2), Err (DecodingError::UnknownFormatType {code: -2} ));
        assert_eq!(decode_type(1), Ok (FormatType::CharT));
        assert_eq!(decode_type(9), Err (DecodingError::UnknownFormatType {code: 9} ));
    }

}
