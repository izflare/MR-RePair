extern crate bit_vec;
extern crate strlib;

use bit_vec::BitVec;
use std::time::Instant;
use super::{cfg::*};
use super::{poppt::*};
use strlib::ffenc;

pub fn encode(g: &Grammar, bv: &mut BitVec) -> () {

    fn adaptive_enc(v: &Vec<u32>, init: u32, bv: &mut BitVec) -> () {
        let mut m: u32 = 0;
        for e in v {if *e > m {m = *e;}}
        ffenc::to_bits(32 - m.leading_zeros(), 32, bv);
        let mut r = 32 - init.leading_zeros();
        for i in 0..v.len() {
            ffenc::to_bits(v[i], r, bv);
            if (i as u32 + init) == 2_u32.pow(r) && r < 32 - m.leading_zeros() {r += 1;}
        }
    }

    let start = Instant::now();
    let mut p: POPPT = POPPT::new();
    g.to_poppt(&mut p);
    
    let mut z: BitVec = BitVec::new();
    let mut l: BitVec = BitVec::new();

    ffenc::encode(&p.terminal.iter().map(|x| *x as u32).collect::<Vec<u32>>(), &mut z);
    adaptive_enc(&p.label, p.terminal.len() as u32 + 1, &mut l);
    ffenc::to_bits(z.len() as u32, 32, bv);
    for b in z {bv.push(b);}
    ffenc::to_bits(p.bit.len() as u32, 32, bv);
    for b in &p.bit {bv.push(b);}
    for b in &l {bv.push(b);}

    let end = start.elapsed();
    println!("[Result: bit encoding]");
    println!("POPPT bit vec len : {:?}", p.bit.len());
    println!("Label length      : {:?}", p.label.len());
    println!("Total bit length  : {:?} [bits]", bv.len());
    println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);

}


pub fn decode(bv: &BitVec, g: &mut Grammar) -> () {

    fn adaptive_dec(bv: &BitVec, init: u32, v: &mut Vec<u32>) -> () {
        let mut r = 32 - init.leading_zeros();
        let mut sum = 32;
        let mut m: u32 = 0;
        let mut u: u32 = 0;
        for i in 0..bv.len() {
            if i < 32 {m <<= 1; if bv[i] {m += 1;}}
            else {
                u <<= 1; if bv[i] {u += 1;}
                if (i as u32 - sum) % r == (r - 1) {
                    v.push(u); 
                    u = 0;
                    if (v.len() as u32 + init) == 2_u32.pow(r) && r < m {r += 1; sum = i as u32;}
                }
            }
        }
    }

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
    adaptive_dec(&l, p.terminal.len() as u32 + 1, &mut p.label);

    p.to_grammar(g);

}

