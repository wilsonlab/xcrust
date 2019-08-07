use std::collections::{HashMap};
use crate::mwl_ad;
use std::fmt;
use crate::num::Float;
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


pub fn output_type(p: &Param) -> mwl_ad::FormatType {
    match p {
        Param::T_PX    | Param::T_PY  | Param::T_PA    | Param::T_PB    |
        Param::POS_X   | Param::POS_Y | Param::T_MAXWD | Param::T_MAXHT |
        Param::T_TPX   | Param::T_TPY | Param::T_TPA   | Param::T_TPB
            => mwl_ad::FormatType::ShortT,
        Param::TIME
            => mwl_ad::FormatType::DoubleT,
        Param::TIMESTAMP
            => mwl_ad::FormatType::ULongT,
        _
            => mwl_ad::FormatType::FloatT,
    }
}

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
                    hm.insert( format!("{:?}",param).to_lowercase(), param );
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

#[derive (Clone, Copy, Debug, PartialEq)]
enum ParamValue
{
    PInt    (i32),
    PShort  (i16),
    PFloat  (f32),
    PDouble (f64),
}

fn compute_params( params: Vec<Param>,
                   spike:  &Spike<f32,f32>,
                   pos:    &DiodePos<f32,f32>) -> Vec<ParamValue>
{
    let mut cache = HashMap::new();
    params.into_iter().map(|p| compute_param(p, &mut cache, spike, pos)).collect()
}

// Associate params with functions from (spike, pos) to some value
// TODO: Can we somehow replace this with traits?
fn compute_param( param: Param,
                  mut cache: &mut ParamCache,
                  spike: &Spike<f32, f32>,
                  pos:   &DiodePos<f32, f32> ) -> ParamValue {
    match param {
        Param::TIME  => ParamValue::PDouble(spike.time as f64),
        Param::POS_X => pos_x_param( pos ),
        Param::POS_Y => pos_y_param( pos ),
        Param::T_PX  => tetrode_amplitude( spike, 0, &mut cache),
        Param::T_PY  => tetrode_amplitude( spike, 1, &mut cache),
        Param::T_PA  => tetrode_amplitude( spike, 2, &mut cache),
        Param::T_PB  => tetrode_amplitude( spike, 3, &mut cache),
        _ => unimplemented!()
    }
}

fn pos_x_param(pos: &DiodePos<f32,f32>) -> ParamValue {
    let v = ((pos.diode_front.0 + pos.diode_back.0)*0.5).round();
    ParamValue::PShort( v as i16 )
}

fn pos_y_param(pos: &DiodePos<f32,f32>) -> ParamValue {
    let v = ((pos.diode_front.1 + pos.diode_back.1)*0.5).round();
    ParamValue::PShort( v as i16 )
}

// For a given tetrode and channel, get that channels amplitude
// at the time when the whole tetrode is at its global maximum
// If the requested channel happens to contain the global maximum,
// then the result is the index of this channel's maximum.
// Often though, the global maximum occurs on a different channel.
// Since this parameter fetching function will usually be called
// multiple times (e.g. 4 times for a tetrode) for a single spike,
// we allow the global-maximum to be cached (otherwise we would do
// the same global-max search e.g. 4 times
fn tetrode_amplitude( // param: Param,
                     spike: &Spike<f32, f32>,
                     channel: usize,
                     mut cache: &mut ParamCache) -> ParamValue {
    let get_global_max = || {
        let mut max_so_far = f32::min_value();
        let mut ind_so_far = 0;
        let n_chan = spike.waveforms.len();
        for chan in 0..n_chan {
            let this_samps = &spike.waveforms[chan];
            for ind in 0..(this_samps.len()) {
                if this_samps[ind] > max_so_far {
                    max_so_far = this_samps[ind];
                    ind_so_far = ind;
                };
            }
        };
        ParamValue::PInt(ind_so_far as i32)
        };

    match cache_value("t_v_max", get_global_max, &mut cache) {
        ParamValue::PInt(global_max_time)
            => ParamValue::PFloat( spike.waveforms[channel][global_max_time as usize] ),
        _ => panic!("Impossible case!")
    }
}

type ParamCache<'a> = HashMap<&'static str, (u32, ParamValue)>;

// TODO: Can this be done with less copying?
fn cache_value<'a>( cache_key: &'static str,
                mut compute_cache_value: (impl FnMut() -> ParamValue),
                cache: &'a mut ParamCache<'a>
) -> ParamValue {
    let cached_v = cache.get(cache_key);
    match cached_v {
        Some(&(n,v)) => {
            cache.insert(cache_key, (n+1,v));
            v.clone()
        },
        None => {
            let v = compute_cache_value();
            cache.insert(cache_key, (0,v));
            v.clone()
        }
    }
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

    #[test]
    fn it_caches() {
        let mut cache = HashMap::new();
        let get_int = || {
            ParamValue::PInt(3)
        };
        let x = cache_value("x", get_int, &mut cache);
        let y = cache_value("x", get_int, &mut cache);
        let n = cache.get("x").map(|&(n,_)| n);
        assert_eq!(x, ParamValue::PInt(3));
        assert_eq!(y, x);
        assert_eq!(n, Some(1));
    }

    #[test]
    fn it_computes_spike_params() {
        let test_spike =
            Spike { time: 100.00,
                    waveforms: vec![
                        vec![0.0, 0.1, 0.9, 0.6, 0.5, -0.2, -0.1, 0.0],
                        vec![0.0, 0.9, 0.9, 0.9, 0.9,  0.4, -0.2, 0.0],
                        vec![0.0, 1.0, 0.9, 0.9, 0.9,  0.4, -0.2, 0.0],
                        vec![0.0, 0.9, 0.9, 0.9, 0.9,  0.4, -0.2, 0.0]
                        //        ^^^-- Here is the global-max time
                    ],
            };
        let test_pos =
            DiodePos { time: 99.9,
                       diode_front: (2.0,1.0),
                       diode_back:  (8.0,3.0)
            };
        let params = vec![Param::TIME, Param::POS_X, Param::POS_Y,
                          Param::T_PX, Param::T_PY,  Param::T_PA,  Param::T_PB];
        let spike_params = compute_params( params, &test_spike, &test_pos );
        assert_eq!(spike_params,
                   vec![ParamValue::PDouble(100.00),
                        ParamValue::PShort(5),
                        ParamValue::PShort(2),
                        ParamValue::PFloat(0.1),
                        ParamValue::PFloat(0.9),
                        ParamValue::PFloat(1.0),
                        ParamValue::PFloat(0.9)
                   ]);
    }

}
