extern crate bit_vec;
extern crate strlib;

use bit_vec::BitVec;
use super::super::{cfg::*};
use super::super::{poppt::*};
use strlib::fble;
use strlib::packed_gamma;

pub fn encode(g: &Grammar, bv: &mut BitVec, blocksize: u32) -> () {

    let mut p: POPPT = POPPT::new();
    g.to_poppt(&mut p);
    
    let mut z: BitVec = BitVec::new();
    let mut l: BitVec = BitVec::new();

    fble::encode(&p.terminal.iter().map(|x| *x as u32).collect::<Vec<u32>>(), &mut z);
    packed_gamma::encode(&p.label, blocksize, &mut l);
    fble::to_bv(z.len() as u32, 32, bv);
    for b in z {bv.push(b);}
    fble::to_bv(p.bit.len() as u32, 32, bv);
    for b in &p.bit {bv.push(b);}
    for b in &l {bv.push(b);}

    println!("[Result: bit encoding]");
    println!("POPPT bit vec len : {:?}", p.bit.len());
    println!("Label length      : {:?}", p.label.len());
    println!("Block size        : {:?}", blocksize);

}


pub fn decode(bv: &BitVec, g: &mut Grammar) -> () {

    let mut p: POPPT = POPPT::new();
    let mut z: BitVec = BitVec::new();
    let mut l: BitVec = BitVec::new();
    let mut zlen: usize = 0;
    let mut blen: usize = 0;
    let mut v: Vec<u32> = Vec::new();
    for i in 8..bv.len() {
        if i < 8 + 32 {zlen <<= 1; if bv[i] {zlen += 1;}}
        else if i < 8 + 32 + zlen {z.push(bv[i]);}
        else if i < 8 + 64 + zlen {blen <<= 1; if bv[i] {blen += 1;}}
        else if i < 8 + 64 + zlen + blen {p.bit.push(bv[i]);}
        else {l.push(bv[i]);}
    }

    fble::decode(&z, &mut v);
    p.terminal = v.iter().map(|x| *x as u8).collect::<Vec<u8>>();
    packed_gamma::decode(&l, &mut p.label);

    p.to_grammar(g);
}

