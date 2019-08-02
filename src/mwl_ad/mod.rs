pub mod header;

#[derive(Debug, FromPrimitive, PartialEq, ToPrimitive)]
pub enum FormatType {
    InvalidT = 0,
    CharT    = 1, // u8
    ShortT   = 2, // i16
    IntT     = 3, // i32
    FloatT   = 4, // f32
    DoubleT  = 5, // f64
    FuncT    = 6,
    FFuncT   = 7,
    ULongT   = 8, // u64
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
