use std::collections::{HashMap};
use crate::spike::{Spike};
use crate::pos::{DiodePos};

#[allow(non_camel_case_types)]
#[derive(Debug, FromPrimitive, PartialEq, ToPrimitive)]
pub enum Param {

    // Bravewave Parameters
    PX = 0,
    PY = 1,
    VX = 2,
    VY = 3,
    TPX = 4,
    TPY = 5,
    TVX =  6,
    TVY  = 7,
    MAXHT =  8,
    PPHS  =  9,
    VPHS  =  10,
    DLAT  =  11,
    MAXWD =  12,

    /*
     ** --------------------------------------
     ** additional non-Brainwave parameters
     ** --------------------------------------
     */
    /*
     ** peak relative amplitude 
     */
    XM4 =  13,
    YM4 =  14,
    XM2 =  15,
    YM2 =  16,
    X0  =  17,
    Y0  =  18,
    XP2 =  19,
    YP2 =  20,
    XP4 =  21,
    YP4 =  22,
    XP8 =  23,
    YP8 =  24,
    XP12  =  25,
    YP12  =  26,
    /*
     ** peak time
     */
    T0  =  27,
    /*
     ** peak relative phase 
     */
    PHSM4 =  28,
    PHSM2 =  29,
    PHS0  =  30,
    PHSP2 =  31,
    PHSP4 =  32,
    PHSP8 =  33,
    PHSP12  =  34,
    
    /*
     ** mean phase and phase variance
     */
    AVGPHS  =  35,
    SDPHS  = 36,
    PAMP  =  37,
    /* 
     ** zero crossing time relative to peak 
     */
    ZCROSS =   38,
    /* 
     ** integral ratios relative to zero crossing 
     */
    INZPM  = 39,

    INZP2M2 =  40,
    INZP4M4 =  41,
    INZP8M8 =  42,

    /* 
     ** integral relative to peak 
     */
    INPM3M6  = 43,
    INPP12M12 = 44,

    /*
     ** template matches
     */


    TEMPLATE0 = 45,
    TEMPLATE1 = 46,
    TEMPLATE2 = 47,
    TEMPLATE3 = 48,
    TEMPLATE4 = 49,
    TEMPLATE5 = 50,
    TEMPLATE6 = 51,
    TEMPLATE7 = 52,
    TEMPLATE8 = 53,
    TEMPLATE9 = 54,

    /*
     ** peak magnitude
     */
    PMAG   = 55,
    VMAG   = 56,

    /*
     ** --------------------------------------
     ** Tetrode parameters  (x,y,,u,v channels)
     ** --------------------------------------
     */
    T_PX  =   70,
    T_PY  =   71,
    T_PA  =   72,
    T_PB  =   73,

    T_VX  =   74,
    T_VY  =   75,
    T_VA  =   76,
    T_VB  =   77,
    
    T_MAXWD  = 78,
    /*
     ** refined tetrode parameters
     */
    T_PMAG   = 79,
    T_VMAG   = 80,
    T_PPHSRXY = 81,
    T_PPHSRXYA  = 82,
    T_PPHSRXYAB = 83,

    T_VPHSRXY = 84,
    T_VPHSRXYA  = 85,
    T_VPHSRXYAB = 86,

    /*
     ** extra tetrode parameters
     */
    T_TPX  = 90,
    T_TPY  = 91,
    T_TPA  = 92,
    T_TPB  = 93,

    T_TVX  = 94,
    T_TVY  = 95,
    T_TVA  = 96,
    T_TVB  = 97,

    T_PPHSXY = 100,
    T_PPHSAB = 101,
    T_PPHSAX = 102,
    T_PPHSAY = 103,
    T_PPHSBX = 104,
    T_PPHSBY = 105,

    T_VPHSXY  = 106,
    T_VPHSAB  = 107,
    T_VPHSAX  = 108,
    T_VPHSAY  = 109,
    T_VPHSBX  = 110,
    T_VPHSBY  = 111,

    T_MAXHT  =  112,


    /*
     ** spatial location parameters
     */
    POS_X   = 113,
    POS_Y  =  114,

    /*
     ** time
     */
    TIME   =  115,
    TIMELO  =   116,
    TIMESTAMP = 117,
    VEL = 118,

    /* 
     ** tetrode integral relative to peak 
     */
    T_INPP3M3  =  119,
    T_INPP3M6  =  120,
    T_INZPM3   =  121,
    T_INZPM6   =  122,
    T_INZ6X    =  123,
    T_INZ6Y    =  124,
    T_INZ6A    =  125,
    T_INZ6B    =  126,
    T_INPP     =  127,
    T_INMAX    =  128,
    T_TPMAX    =  129,
    T_TVMAX    =  130,

    /*  Hadamard transformation axes */
    T_H1      =   131,
    T_H2      =   132,
    T_H3      =   133,
    T_H4      =   134,
}

const MAXTEMPLATES    : i32 = 10;
const MAXTEMPLATESIZE : i32 = 64;

/* 
 ** tetrode integral relative to peak 
 ** They are interchangeable with non-tetrode counterparts
 */
const T_INPM3M6   : Param =  Param::INPM3M6;
const T_INPP12M12 : Param =  Param::INPP12M12;

// There are very many variants in the Param enum, so it would
// require a lot of error-prone typing to create a function for
// parsing command-line strings into the variants.
//
// Instead, we use the fact that numbers are associated to Params
// and the fact that Params can be printed to construct a lookup
// table. Thanks to `lazy_static`, the table is created the first
// time it is used, and is available for reuse on subsequent calls
lazy_static!{
    static ref PARAM_LOOKUP_TABLE : HashMap < String, Param > = {
        let mut hm = HashMap::new();
        for i in 0..140{
            match num::FromPrimitive::from_i32(i) {
                Some( param ) => {
                    hm.insert( format!("{:?}",param).to_lowercase() , param );
                },
                _ => {}
            };
        }
        hm
    };
}

// Interpret a cli flags like `t_py' into a params like Param::T_PY
fn parse_param(i : &str) -> Option<&Param> {
    PARAM_LOOKUP_TABLE.get( &i.to_owned().to_lowercase() ).clone()
}

enum ParamValue
{
    PInt   (u32),
    PFloat (f32),
}

// Associate params with functions from (spike, pos) to some value
// TODO: Can we somehow replace this with traits?
fn compute_param( param: Param,
                  spike: Spike<f32, f32>,
                  pos_now: DiodePos<f32, f32> ) -> ParamValue {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_t_px() {
        assert_eq!( parse_param("t_px"), Some(&Param::T_PX) );
    }

    #[test]
    fn it_fails_parsing_nonsense() {
        assert_eq!( parse_param("t_nonsense"), None);
    }
}
