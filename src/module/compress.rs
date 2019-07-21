extern crate bit_vec;

use std::collections::HashMap;
use std::collections::VecDeque;
use bit_vec::BitVec;
use super::encode;
use super::{ds::*};
use super::{cfg::*};

pub fn compress(s: &Vec<u8>, g: &mut Grammar, minfreq: usize) -> () {

    // preprocessing
    let mut a: Vec<Bucket> = vec![Bucket {val: None, prev: None, next: None}; s.len()];
    let mut h: HashMap<Bigram, *mut Record> = HashMap::with_capacity(s.len());
    let mut f: usize = a.create(&mut h, &mut g.terminal, &s);
    let mut q: Vec<List> = vec![List {head: None, tail: None}; f + 1];
    q.create(&h);

    // algorithm
    let mut v: usize = g.terminal.len() + 1;
    loop {
        if f < minfreq {break;}
        if let Some(r) = q.top(f) { unsafe {
            // the most frequent bigram
            let bg: Bigram = a.rgh_bg((*r).loc).unwrap();
            q.dequeue(r);
            h.remove(&bg);

            // the most frequent maximal repeat
            //{{{
            let mut i: usize = (*r).loc;
            let wth: usize = a.rgh_pos(i).unwrap() - i;
            let mut mr: VecDeque<u32> = VecDeque::new();
            mr.push_back(bg.left);
            mr.push_back(bg.right);
            let mut lft_wth: usize = 0;
            let mut rgh_wth: usize = wth;
            let mut lft_wth_vec: Vec<usize> = vec![0];
            let mut rgh_wth_vec: Vec<usize> = vec![wth];
            let mut rgh_most: *mut Record = r;
            let mut occ: Vec<usize> = Vec::new();
            loop {
                occ.push(i);
                if let Some(next) = a[i].next {i = next;}
                else {break;}
            }
            // left maximal
            loop {
                if let Some(lft_bg) = a.lft_bg(occ[0] - lft_wth) {
                    let mut maximal: bool = true;
                    for i in &occ {
                        if let Some(lft_bg_tmp) = a.lft_bg(i - lft_wth) {
                            if lft_bg_tmp != lft_bg {maximal = false; break;}
                        }
                        else {maximal = false; break;}
                    }
                    if maximal {
                        lft_wth = occ[0] - a.lft_pos(occ[0] - lft_wth).unwrap();
                        lft_wth_vec.push(lft_wth);
                        mr.push_front(a[occ[0] - lft_wth].val.unwrap());
                        q.dequeue(*h.get(&lft_bg).unwrap());
                        h.remove(&lft_bg);
                    }
                    else {break;}
                }
                else {break;}
            }
            // right maximal
            loop {
                if let Some(rgh_bg) = a.rgh_bg(occ[0] + rgh_wth) {
                    let mut maximal: bool = true;
                    for i in &occ {
                        if let Some(rgh_bg_tmp) = a.rgh_bg(i + rgh_wth) {
                            if rgh_bg_tmp != rgh_bg {maximal = false; break;}
                        }
                        else {maximal = false; break;}
                    }
                    if maximal {
                        rgh_wth = a.rgh_pos(occ[0] + rgh_wth).unwrap() - occ[0];
                        rgh_wth_vec.push(rgh_wth);
                        mr.push_back(a[occ[0] + rgh_wth].val.unwrap());
                        let r = *h.get(&rgh_bg).unwrap();
                        q.dequeue(r);
                        rgh_most = r;
                        h.remove(&rgh_bg);
                    }
                    else {break;}
                }
                else {break;}
            }
            let mut wth_vec: Vec<usize> = Vec::new();
            for e in lft_wth_vec.iter().rev() {if lft_wth != *e {wth_vec.push(lft_wth - *e);}}
            for e in &rgh_wth_vec {wth_vec.push(*e + lft_wth);}
            // exclude right most if left most == right most
            if mr[0] == mr[mr.len() - 1] && mr.len() > 2 {
                h.insert(a.rgh_bg((*rgh_most).loc).unwrap(), rgh_most);
                q.enqueue(rgh_most);
                wth_vec.pop();
                mr.pop_back();
            }
            //}}}

            g.rule.push(mr.iter().map(|x| *x).collect::<Vec<u32>>());
            let mut il: usize = (*r).loc - lft_wth;
            let mut ir: usize = il + wth_vec[wth_vec.len() - 1];

            // decrement
            loop {
                //{{{
                // left bigram
                if let Some(lft_bg) = a.lft_bg(il) {
                    if let Some(lft_rec) = h.get(&lft_bg) {
                        let lft_rec = *lft_rec;
                        let lft_pos = a.lft_pos(il).unwrap();
                        if let Some(prev) = a[lft_pos].prev {a[prev].next = a[lft_pos].next;}
                        if let Some(next) = a[lft_pos].next {a[next].prev = a[lft_pos].prev;}
                        q.dequeue(lft_rec);
                        (*lft_rec).freq -= 1;
                        if (*lft_rec).freq < 2 {h.remove(&lft_bg);}
                        else {
                            if (*lft_rec).loc == lft_pos {(*lft_rec).loc = a[lft_pos].next.unwrap();}
                            q.enqueue(lft_rec);
                        }
                        a[lft_pos].prev = None;
                        a[lft_pos].next = None;
                    }
                }

                // right bigram
                if let Some(rgh_bg) = a.rgh_bg(ir) {
                    if let Some(rgh_rec) = h.get(&rgh_bg) {
                        let rgh_rec = *rgh_rec;
                        if let Some(prev) = a[ir].prev {a[prev].next = a[ir].next;}
                        if let Some(next) = a[ir].next {a[next].prev = a[ir].prev;}
                        q.dequeue(rgh_rec);
                        (*rgh_rec).freq -= 1;
                        if (*rgh_rec).freq < 2 {h.remove(&rgh_bg);}
                        else {
                            if (*rgh_rec).loc == ir {(*rgh_rec).loc = a[ir].next.unwrap();}
                            q.enqueue(rgh_rec);
                        }
                        a[ir].prev = None;
                        a[ir].next = None;
                    }
                }
                // replace bigram -> variable and go to next occ
                let jump = match a[il].next {Some(next) => a[next].next, None => None,};
                a[il].val = Some(v as u32);
                for wth in &wth_vec {a[il + wth].val = None;}
                a[il + 1].next = a.rgh_pos(ir);
                if let Some(rgh_ir) = a.rgh_pos(ir) {a[rgh_ir - 1].prev = Some(il);}
                if let Some(next) = a[il].next {
                    if next != ir {
                        il = next; ir = il + wth_vec[wth_vec.len() - 1];
                    }
                    else {
                        if let Some(skip) = jump {
                            a[il].next = Some(skip); a[skip].prev = Some(il);
                            il = skip; ir = il + wth_vec[wth_vec.len() - 1];
                        }
                        else {break;}
                    }
                }
                else {break;}
                //}}}
            }

            // increment
            loop {
                //{{{
                // right bigram
                if let Some(rgh_bg) = a.rgh_bg(il) {
                    if let Some(rgh_rec) = h.get(&rgh_bg) {
                        let rgh_rec = *rgh_rec;
                        if (*rgh_rec).loc != il {
                            a[il].next = Some((*rgh_rec).loc);
                            a[(*rgh_rec).loc].prev = Some(il);
                            q.dequeue(rgh_rec);
                            (*rgh_rec).freq += 1;
                            (*rgh_rec).loc = il;
                            q.enqueue(rgh_rec);
                        }
                        else {
                            if let Some (true_prev) = a[(*rgh_rec).loc].next {
                                a[true_prev].prev = Some(il);
                            }
                        }
                    }
                    else {
                        let rgh_rec = Box::into_raw(Box::new(Record {loc: il, freq: 1, prev: None, next: None}));
                        h.insert(rgh_bg, rgh_rec);
                        q.enqueue(rgh_rec);
                        a[il].next = None;
                    }
                }

                // left bigram
                if let Some(lft_bg) = a.lft_bg(il) {
                    let lft_pos = a.lft_pos(il).unwrap();
                    if let Some(lft_rec) = h.get(&lft_bg) {
                        let lft_rec = *lft_rec;
                        a[lft_pos].next = Some((*lft_rec).loc);
                        a[(*lft_rec).loc].prev = Some(lft_pos);
                        q.dequeue(lft_rec);
                        (*lft_rec).freq += 1;
                        (*lft_rec).loc = lft_pos;
                        q.enqueue(lft_rec);
                    }
                    else {
                        let lft_rec = Box::into_raw(Box::new(Record {loc: lft_pos, freq: 1, prev: None, next: None}));
                        h.insert(lft_bg, lft_rec);
                        q.enqueue(lft_rec);
                        a[lft_pos].next = None;
                    }
                }

                // go to prev occ
                if let Some(prev) = a[il].prev {
                    let old = il;
                    il = prev;
                    a[old].prev = None;
                }
                else {break;}
                //}}}
            }

            v += 1;
            if v >= std::u32::MAX as usize {break;}
        }}
        else {f -= 1;}
    }

    for c in &a {match (*c).val {Some(x) => g.sequence.push(x), None => ()}}
}

pub fn decompress(bv: &BitVec, u: &mut Vec<u8>) -> () {
    let mut g: Grammar = Grammar::new();
    encode::decode(bv, &mut g);
    g.derive(u);
}
