extern crate bit_vec;
extern crate strlib;

use bit_vec::BitVec;
use strlib::fixed;
use super::super::{cfg::*};

pub fn encode(g: &Grammar, bv: &mut BitVec) -> () {

    fixed::to_bv(g.terminal.len() as u32, 32, bv);
    for e in &g.terminal {fixed::to_bv(*e as u32, 8, bv);}
    fixed::to_bv(g.rule.len() as u32 + g.rule.iter().fold(0, |sum, x| sum + x.len())  as u32, 32, bv);
    for e in &g.rule {
        fixed::to_bv(e.len() as u32, 32, bv);
        for f in e {
            fixed::to_bv(*f as u32, 32, bv);
        }
    }
    for e in &g.sequence {fixed::to_bv(*e, 32, bv);}

}


pub fn decode(bv: &BitVec, g: &mut Grammar) -> () {

    let mut zlen = 0;
    let mut z: u8 = 0;
    let mut vlen = 0;
    let mut a: u32 = 0;
    let mut s: u32 = 0;
    let mut sum = 40;
    let mut block = 0;
    let mut v: Vec<u32> = Vec::new();

    for i in 8..bv.len() {
        if i < 8 + 32 {zlen <<= 1; if bv[i] {zlen += 1;}}
        else if i < 8 + 32 + zlen * 8 {
            z <<= 1; if bv[i] {z += 1;}
            if (i - sum) % 8 == 7 {g.terminal.push(z); z = 0; sum = i + 1;}
        }
        else if i < 8 + 32 * 2 + zlen * 8 {vlen <<= 1; if bv[i] {vlen += 1;}}
        else if i < 8 + 32 * 2 + zlen * 8 + vlen * 32 {
            if block == 0 {
                a <<= 1; if bv[i] {a += 1;}
                if (i - sum) % 32 == 31 {block = a; a = 0;}
            }
            else {
                a <<= 1; if bv[i] {a += 1;}
                if (i - sum) % 32 == 31 {
                    v.push(a); a = 0; block -= 1; 
                    if block == 0 {
                        g.rule.push(v);
                        v = Vec::new();
                    }
                }
            }
        }
        else {
            s <<= 1; if bv[i] {s += 1;}
            if (i - sum) % 32 == 31 {g.sequence.push(s);}
        }
    }

}
