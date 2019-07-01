extern crate bit_vec;
extern crate strlib;

pub mod u32bits;
pub mod fble;
pub mod poppt;

use bit_vec::BitVec;
use std::time::Instant;
use strlib::fixed;
use crate::module::{cfg::*};
use super::encode;

pub fn encode(g: &Grammar, mode: &str, bv: &mut BitVec) -> () {

    println!("[Bit encoding]");
    println!("Encoding mode     : {}", mode);

    let start = Instant::now();

    if mode == "u32bits" {
        fixed::to_bv(0, 8, bv);
        encode::u32bits::encode(g, bv);
    }
    else if mode == "fixed" {
        fixed::to_bv(1, 8, bv);
        encode::fble::encode(g, bv);
    }
    else if mode == "POPPT" {
        fixed::to_bv(2, 8, bv);
        encode::poppt::encode(g, bv);
    }
    else {panic!("encoding mode error");}

    let end = start.elapsed();

    println!("Bit length        : {:?} [bits]", bv.len());
    println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);
}


pub fn decode(bv: &BitVec, g: &mut Grammar) -> () {

    let mut mode = "";
    let mut mode_number: u32 = 0;
    for i in 0..8 {mode_number <<= 1; if bv[i] {mode_number += 1;}}
    assert!(mode_number <= 2, "unknown encoding mode");

    if mode_number == 0 {
        mode = "u32bits";
        encode::u32bits::decode(bv, g);
    }
    else if mode_number == 1 {
        mode = "fixed";
        encode::fble::decode(bv, g);
    }
    else if mode_number == 2 {
        mode = "POPPT";
        encode::poppt::decode(bv, g);
    }

    println!("Encoding mode : {}", mode);

}
