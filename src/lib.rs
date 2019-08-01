#[macro_use] extern crate num;
#[macro_use] extern crate num_derive;
#[macro_use] extern crate failure;
pub mod spike;
pub mod mwl_ad;

pub fn hi(x : i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(hi(3), 4);
    }
}
