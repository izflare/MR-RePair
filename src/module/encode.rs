extern crate bit_vec;
extern crate strlib;

use bit_vec::BitVec;
use std::time::Instant;
use super::{cfg::*};
use super::{poppt::*};
use strlib::ffenc;

pub fn encode(g: &Grammar, bv: &mut BitVec) -> () {

    let start = Instant::now();
    let mut p: POPPT = POPPT::new();
    g.to_poppt(&mut p);
    
    let mut z: BitVec = BitVec::new();
    let mut l: BitVec = BitVec::new();

    ffenc::encode(&p.terminal.iter().map(|x| *x as u32).collect::<Vec<u32>>(), &mut z);
    ffenc::encode(&p.label, &mut l);
    ffenc::to_bits(z.len() as u32, 32, bv);
    for b in z {bv.push(b);}
    ffenc::to_bits(p.bit.len() as u32, 32, bv);
    for b in &p.bit {bv.push(b);}
    for b in l {bv.push(b);}

    let end = start.elapsed();
    println!("[Result: bit encoding]");
    println!("POPPT bit vec len : {:?}", p.bit.len());
    println!("Label length      : {:?}", p.label.len());
    println!("Total bit length  : {:?} [bits]", bv.len());
    println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);
}


pub fn decode(bv: &BitVec, g: &mut Grammar) -> () {

    let mut p: POPPT = POPPT::new();
    let mut z: BitVec = BitVec::new();
    let mut l: BitVec = BitVec::new();
    let mut zlen: usize = 0;
    let mut blen: usize = 0;
    let mut v: Vec<u32> = Vec::new();
    for i in 0..bv.len() {
        if i < 32 {zlen <<= 1; if bv[i] {zlen += 1;}}
        else if i < 32 + zlen {z.push(bv[i]);}
        else if i < 64 + zlen {blen <<= 1; if bv[i] {blen += 1;}}
        else if i < 64 + zlen + blen {p.bit.push(bv[i]);}
        else {l.push(bv[i]);}
    }

    ffenc::decode(&z, &mut v);
    p.terminal = v.iter().map(|x| *x as u8).collect::<Vec<u8>>();
    ffenc::decode(&l, &mut p.label);

    p.to_grammar(g);
}

