use std::collections::HashMap;
use std::cmp::{max, Ordering};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Bigram {pub left: u32, pub right: u32,}

impl Ord for Bigram {
    fn cmp(&self, other: &Self) -> Ordering {
        return max(self.left, self.right).cmp(&max(other.left, other.right));
    }
}

impl PartialOrd for Bigram {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

#[derive(Clone, Debug)]
pub struct Bucket {pub val: Option<u32>, pub prev: Option<usize>, pub next: Option<usize>,}

pub trait PairArray: std::ops::Index<usize> {
    fn lft_pos(&self, i: usize) -> Option<usize>;
    fn rgh_pos(&self, i: usize) -> Option<usize>;
    fn lft_bg(&self, i: usize) -> Option<Bigram>;
    fn rgh_bg(&self, i: usize) -> Option<Bigram>;
    fn create(&mut self, h: &mut HashMap<Bigram, *mut Record>, z: &mut Vec<u8>, s: &Vec<u8>) -> usize;
    fn print(&self) -> ();
}

impl PairArray for Vec<Bucket> {
    //{{{
    fn print(&self) -> () {
        for i in 0..self.len() {
            println!("{:2}: {:?}", i, self[i]);
        }
        println!("");
    }

    fn lft_pos(&self, i: usize) -> Option<usize> {
        if i == 0 {None}
        else if self[i - 1].val == None {self[i - 1].prev}
        else {Some(i - 1)}
    }

    fn rgh_pos(&self, i: usize) -> Option<usize> {
        if i >= self.len() - 1 {None}
        else if self[i + 1].val == None {self[i + 1].next}
        else {Some(i + 1)}
    }

    fn lft_bg(&self, i: usize) -> Option<Bigram> {
        if let Some(lft) = self.lft_pos(i) {
            match (self[lft].val, self[i].val) {
                (Some(x), Some(y)) => Some(Bigram {left: x, right: y,}),
                _ => None
            }
        }
        else {None}
    }

    fn rgh_bg(&self, i: usize) -> Option<Bigram> {
        if let Some(rgh) = self.rgh_pos(i) {
            match (self[i].val, self[rgh].val) {
                (Some(x), Some(y)) => Some(Bigram {left: x, right: y,}),
                _ => None
            }
        }
        else {None}
    }

    fn create(&mut self, h: &mut HashMap<Bigram, *mut Record>, z: &mut Vec<u8>, s: &Vec<u8>) -> usize {
        let mut d: HashMap<u8, u32> = HashMap::new();
        let mut var: u32 = 0;
        let mut f: usize = 1;
        for i in 0..self.len() {
            if !d.contains_key(&s[i]) {
                z.push(s[i]);
                var += 1;
            }
            self[i].val = Some(*d.entry(s[i]).or_insert(var));
        }
        for i in (0..self.len()-1).rev() {
            if let Some(bg) = self.rgh_bg(i) {
                if h.contains_key(&bg) {
                    if let Some(r) = h.get(&bg) { 
                        unsafe {
                            self[i].next = Some((**r).loc);
                            self[(**r).loc].prev = Some(i);
                            (**r).loc = i;
                            (**r).freq += 1;
                            if f < (**r).freq {f = (**r).freq;}
                        }
                    }
                }
                else {h.insert(bg, Box::into_raw(Box::new(Record {loc: i, freq: 1, prev: None, next: None})));}
            }
        }
        return f;
    }
    //}}}
}

#[derive(Debug)]
pub struct Record {pub loc: usize, pub freq: usize, pub prev: Option<*mut Record>, pub next: Option<*mut Record>}

#[derive(Clone)]
pub struct List {pub head: Option<*mut Record>, pub tail: Option<*mut Record>}

pub trait FreqTable: std::ops::Index<usize> {
    fn top(&self, f: usize) -> Option<*mut Record>;
    unsafe fn enqueue(&mut self, r: *mut Record) -> ();
    unsafe fn dequeue(&mut self, r: *mut Record) -> ();
    fn create(&mut self, h: &HashMap<Bigram, *mut Record>) -> ();
    unsafe fn sort(&mut self, f: usize, a: &Vec<Bucket>) -> ();
}

impl FreqTable for Vec<List> {
    //{{{
    fn top(&self, f: usize) -> Option<*mut Record> {
        return self[f].head
    }

    unsafe fn enqueue(&mut self, r: *mut Record) -> () {
        let f = (*r).freq;
        if let Some(tail) = self[f].tail {
            (*tail).next = Some(r);
            (*r).prev = Some(tail);
            self[f].tail = Some(r);
        }
        else {
            self[f].head = Some(r);
            self[f].tail = Some(r);
        }
    }

    unsafe fn dequeue(&mut self, r: *mut Record) -> () {
        let f = (*r).freq;
        match ((*r).prev, (*r).next) {
            (Some(x), Some(y)) => {(*x).next = Some(y); (*y).prev = Some(x);},
            (Some(x), None) => {(*x).next = None; self[f].tail = Some(x);},
            (None, Some(y)) => {(*y).prev = None; self[f].head = Some(y);},
            (None, None) => {self[f].head = None; self[f].tail = None;},
        }
        (*r).prev = None;
        (*r).next = None;
    }

    fn create(&mut self, h: &HashMap<Bigram, *mut Record>) -> () {
        for r in h.values() {
            unsafe {
                if (*(*r)).freq > 1 {self.enqueue(*r);}
            }
        }
    }

    unsafe fn sort(&mut self, f: usize, a: &Vec<Bucket>) -> () {
        let mut v: Vec<*mut Record> = Vec::new();
        loop {
            if let Some(r) = self.top(f) {
                self.dequeue(r);
                v.push(r);
            }
            else {break;}
        }
        v.sort_by(|x, y| (a.rgh_bg((**x).loc)).cmp(&a.rgh_bg((**y).loc)));
        for r in &v {self.enqueue(*r);}
    }
    //}}}
}


