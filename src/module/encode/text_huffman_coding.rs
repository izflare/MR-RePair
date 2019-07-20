extern crate bit_vec;
extern crate strlib;

use bit_vec::BitVec;
use strlib::{fble, huffman_coding};
use super::super::{cfg::*};

pub fn encode(g: &Grammar, bv: &mut BitVec) -> () {

    fble::to_bv(g.terminal.len() as u32, 32, bv);
    fble::to_bv(g.rule.len() as u32 + g.rule.iter().fold(0, |sum, x| sum + x.len()) as u32, 32, bv);

    let mut v: Vec<u32> = Vec::new();
    for e in &g.terminal {v.push(*e as u32);}
    for e in &g.rule {
        v.push(e.len() as u32);
        for f in e {v.push(*f);}
    }
    for e in &g.sequence {v.push(*e);}

    let mut tbv = BitVec::new();
    huffman_coding::encode(&v, &mut tbv);
    fble::to_bv(tbv.len() as u32, 32, bv);
    for b in &tbv {bv.push(b);}

}


pub fn decode(bv: &BitVec, g: &mut Grammar) -> () {

    let mut zlen = 0;
    let mut rlen = 0;
    let mut dlen = 0;
    let mut d = BitVec::new();

    for i in 8..bv.len() {
        if i < 8 + 32 {zlen <<= 1; if bv[i] {zlen += 1;}}
        else if i < 8 + 32 * 2 {rlen <<= 1; if bv[i] {rlen += 1;}}
        else if i < 8 + 32 * 3 {dlen <<= 1; if bv[i] {dlen += 1;}}
        else if i < 8 + 32 * 3 + dlen {d.push(bv[i]);}
    }

    let mut v: Vec<u32> = Vec::new();
    huffman_coding::decode(&d, &mut v);

    let mut num = 0;
    let mut block: Vec<u32> = Vec::new();
    for i in 0..v.len() {
        if i < zlen {g.terminal.push(v[i] as u8);}
        else if i < zlen + rlen {
            if num == 0 {num = v[i];}
            else {
                block.push(v[i]);
                num -= 1;
                if num == 0 {g.rule.push(block); block = Vec::new();}
            }
        }
        else {g.sequence.push(v[i]);}
    }

}
