extern crate clap;

use clap::{App, Arg};
use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::time::Instant;

fn main() {

    // 引数処理
    let app = App::new("MR-RePair")
        //{{{
        .version("0.1.0")                       
        .author("flare")     
        .about("MR-RePair Compressor")
        .arg(Arg::with_name("input")
            .help("input sourse text file") 
            .short("i")
            .long("input")
            .takes_value(true)                  
            .required(true)                     
        )
        .arg(Arg::with_name("print")
            .help("print the detail of constructed grammar")
            .short("p")
            .long("print")
        );
        //}}}
    let matches = app.get_matches();

    // 読み込み
    let mut s = Vec::new();
    let mut f = BufReader::new(File::open(&matches.value_of("input").unwrap()).expect("file not found"));
    f.read_to_end(&mut s).expect("Unable to read");
    let start = Instant::now();


    // preprocessing
    // 終端記号を変数に置換して，文字列を配列に格納
    // each tuple is (0: char, 1: prev, 2: next)
    let mut a: Vec<(Option<usize>, Option<usize>, Option<usize>)> = vec![(None, None, None); s.len()]; 
    let mut z: Vec<u8> = Vec::new();
    //{{{
    {
        let mut d: HashMap<u8, usize> = HashMap::new();
        let mut x: usize = 0;
        for i in 0..s.len() {
            if d.contains_key(&s[i]) {
                let e = d.get(&s[i]);
                a[i] = (Some(*e.unwrap()), None, None);
            }
            else {
                d.insert(s[i], x);
                a[i] = (Some(x), None, None);
                x += 1;
                z.push(s[i]);
            }
        }
    }

    // a の関数
    // 右隣の空でない要素の番号を取得
    fn get_rt(a: &Vec<(Option<usize>, Option<usize>, Option<usize>)>, i: usize) -> usize {
        //{{{
        if a[i+1].0 == None {
            match a[i+1].2 {Some(x) => x, None => 0}
        }
        else {
            i+1
        }
        //}}}
    }

    // 左隣の空でない要素の番号を取得
    fn get_lt(a: &Vec<(Option<usize>, Option<usize>, Option<usize>)>, i: usize) -> usize {
        //{{{
        if a[i-1].0 == None {
            a[i-1].1.unwrap()
        }
        else {
            i-1
        }
        //}}}
    }

    // bigramを取得
    fn get_bg(a: &Vec<(Option<usize>, Option<usize>, Option<usize>)>, i: usize) -> (usize, usize) {
        //{{{
        (a[i].0.unwrap(), a[get_rt(&a, i)].0.unwrap())
        //}}}
    }

    //}}}

    // bigramの位置をつなぎながらハッシュ表を作成
    struct Rec {loc: usize, freq: usize, prev: Option<*mut Rec>, next: Option<*mut Rec>};
    let mut h: HashMap<(usize, usize), *mut Rec> = HashMap::new();
    let mut f: usize = 1;
    let mut k: Vec<(usize, usize)> = Vec::new();
    //{{{
    for i in (0..s.len()-1).rev() {
        let b = (a[i].0.unwrap(), a[i+1].0.unwrap());
        if h.contains_key(&b) {
            unsafe {
                let mut r: &mut Rec = &mut **(h.get(&b).unwrap());
                a[i].2 = Some(r.loc);
                a[r.loc].1 = Some(i);
                r.loc = i;
                r.freq += 1;
                if f < r.freq {f = r.freq;}
            }
        }
        else {
            let r = Box::new(Rec {loc: i, freq: 1, prev: None, next: None});
            let x: *mut Rec = Box::into_raw(r);
            h.insert(b, x);
            k.push(b);
        }
    }
    //}}}

    // 頻度表を作成
    let mut q: Vec<Option<*mut Rec>> = vec![None; f+1];
    //{{{
    for e in &k {
        let v = h.get(e).unwrap();
        unsafe {
            let r: &mut Rec = &mut **v;
            in_rec(&mut q, r);
        }
    }

    // q の関数
    // Record をリストから切り離す
    fn out_rec(q: &mut Vec<Option<*mut Rec>>, r: &mut Rec) {
        //{{{
        if r.prev == None {
            q[r.freq] = r.next;
        }
        else {
            unsafe {
                let pr: &mut Rec = &mut *r.prev.unwrap();
                pr.next = r.next;
            }
        }

        if r.next != None {
            unsafe {
                let nx: &mut Rec = &mut *r.next.unwrap();
                nx.prev = r.prev;
            }
        }
        r.prev = None;
        r.next = None;
        //}}}
    }

    // Record をリストの先頭に追加
    fn in_rec(q: &mut Vec<Option<*mut Rec>>, r: &mut Rec) {
        //{{{
        let ptr: *mut Rec = &mut *r;
        if q[r.freq] != None {
            unsafe {
                let nx: &mut Rec = &mut *q[r.freq].unwrap();
                nx.prev = Some(ptr);
            }
            r.next = q[r.freq];
        }
        q[r.freq] = Some(ptr);
        //}}}
    }
    //}}}

    // algorithm
    let mut v: usize = z.len();
    let mut g: Vec<Vec<usize>> = Vec::new();

    while f > 1 {
        if q[f] == None {f -= 1; continue;}
        unsafe {
            // 最頻出ペアを同定
            let mut r: &mut Rec = &mut *q[f].unwrap();
            let mut b = get_bg(&a, r.loc);
            // maximal repeat を探索
            //{{{
            let mut i: usize = r.loc;
            let mut vi:  Vec<usize> = Vec::new();
            loop {
                vi.push(i);
                if a[i].2 == None {break;}
                i = a[i].2.unwrap();
            }
            // right-maximal
            let mut rw: usize = get_rt(&a, i) - i;
            let mut m: bool = false;
            loop {
                if i+rw == 0 || i+rw >= a.len()-1 || get_rt(&a, i+rw) == 0 {break;}
                let e: usize = get_rt(&a, i+rw) - (i+rw);
                let rc = a[i+rw+e].0;
                for x in &vi {
                    if a[x+rw+e].0 != rc {m = true; break;}
                }
                if m {break;}
                let tb = get_bg(&a, i+rw);
                out_rec(&mut q, &mut **(h.get(&tb).unwrap()));
                h.remove(&tb);
                rw += e;
            }
            // left-maximal
            let i: usize = r.loc;
            let mut lw: usize = 0;
            let mut m: bool = false;
            loop {
                if i-lw == 0 {break;}
                let e: usize = (i-lw) - get_lt(&a, i-lw);
                let lc = a[i-lw-e].0;
                for x in &vi {
                    if a[x-lw-e].0 != lc {m = true; break;}
                }
                if m {break;}
                h.remove(&b);
                out_rec(&mut q, &mut r);
                lw += e;
                b = get_bg(&a, i-lw);
                r = &mut **(h.get(&b).unwrap());
            }
            //}}}
            let mut w: usize = lw + rw;

            let mut i: usize = r.loc;
            // maximal repeat の左右端一致時に片端を除外
            let mut p: Vec<usize> = Vec::new();
            for pos in i..i+w+1 {
                if a[pos].0 != None && pos != i && pos != i+w {
                    p.push(pos-i);
                }
            }
            if a[i].0 == a[i+w].0 && p.len() > 0 {
                //{{{
                let dif = get_rt(&a, i) - i;
                w -= dif;
                p = p.into_iter().map(|x| x - dif).collect();
                i = get_rt(&a, i);
                b = get_bg(&a, i);
            }
            else {
                out_rec(&mut q, &mut r);
                //}}}
            }
            let mut mr: Vec<usize> = Vec::new();
            mr.push(a[i].0.unwrap());
            for pos in &p {
                mr.push(a[i+*pos].0.unwrap());
            }
            mr.push(a[i+w].0.unwrap());
            // g.push(vec![b.0, b.1]);
            g.push(mr);

            // 置換・更新，順方向，既存ペアのデクリメント
            let mut o: bool = false;
            loop {
                //{{{
                // 左隣のペアの頻度をデクリメント
                if i > 0 && !o {
                    //{{{
                    // let lt_i = get_lt(&a, i);
                    let lt_i = get_lt(&a, i);
                    let lt_b: (usize, usize) = get_bg(&a, lt_i);
                    let mut lt_r: &mut Rec = &mut **h.get(&lt_b).unwrap();
                    match a[lt_i].1 {Some(x) => a[x].2 = a[lt_i].2, None => ()}
                    match a[lt_i].2 {Some(x) => a[x].1 = a[lt_i].1, None => ()}
                    out_rec(&mut q, &mut lt_r);
                    lt_r.freq -= 1;
                    if lt_r.freq > 0 && lt_r.loc == lt_i {lt_r.loc = a[lt_i].2.unwrap()}
                    if lt_r.freq > 0 {in_rec(&mut q, &mut lt_r);}
                    else {h.remove(&lt_b);}
                    //}}}
                }

                // 右隣のペアの頻度をデクリメント
                // let rt_i = get_rt(&a, i);
                let rt_i = i+w;
                if i+w < a.len()-1 && rt_i != 0 && rt_i < a.len()-1 && get_rt(&a, rt_i) != 0 {
                    //{{{
                    let rt_b: (usize, usize) = get_bg(&a, rt_i);
                    match a[i].2 {
                        Some(x) => {
                            // fully overlap
                            if x == rt_i {
                                let nx_rt_i = a[rt_i].2;
                                a[i].2 = nx_rt_i;
                                match nx_rt_i {
                                    Some(x) => {
                                        a[x].1 = Some(i);
                                        o = get_rt(&a, rt_i) == x;
                                    }, 
                                    None => {o = false;}
                                }
                            }
                            else {
                                let mut rt_r: &mut Rec = &mut **h.get(&rt_b).unwrap();
                                match a[rt_i].1 {Some(x) => a[x].2 = a[rt_i].2, None => ()}
                                match a[rt_i].2 {Some(x) => a[x].1 = a[rt_i].1, None => ()}
                                out_rec(&mut q, &mut rt_r);
                                rt_r.freq -= 1;
                                if rt_r.freq > 0 && rt_r.loc == rt_i {rt_r.loc = a[rt_i].2.unwrap()}
                                if rt_r.freq > 0 {in_rec(&mut q, &mut rt_r);}
                                else {h.remove(&rt_b);}
                                // consider partially overlap
                                o = x == get_rt(&a, rt_i);
                            }
                        },
                        None => {
                            let mut rt_r: &mut Rec = &mut **h.get(&rt_b).unwrap();
                            match a[rt_i].1 {Some(x) => a[x].2 = a[rt_i].2, None => ()}
                            match a[rt_i].2 {Some(x) => a[x].1 = a[rt_i].1, None => ()}
                            out_rec(&mut q, &mut rt_r);
                            rt_r.freq -= 1;
                            if rt_r.freq > 0 && rt_r.loc == rt_i {rt_r.loc = a[rt_i].2.unwrap()}
                            if rt_r.freq > 0 {in_rec(&mut q, &mut rt_r);}
                            else {h.remove(&rt_b);}
                            o = false;
                        }
                    }
                    // ポインタ張り替え
                    let nx_rt_i = get_rt(&a, rt_i);
                    if nx_rt_i != 0 {
                        a[nx_rt_i-1].1 = Some(i);
                        a[i+1].0 = None;
                        a[i+1].2 = Some(nx_rt_i);
                    }
                }
                else {
                    a[i+1].2 = None;
                    o = false;
                    //}}}
                }

                // ペア置換
                // a[i].0 = Some(v);
                a[i].0 = Some(v);
                for pos in &p {if *pos > 0 {a[i+*pos].0 = None;}}
                a[rt_i].0 = None;
                if a[i].2 == None {break;}
                i = a[i].2.unwrap();
            //}}}
            }

            // 逆方向，新出ペアのインクリメント
            o = false;
            loop {
                //{{{
                // 右隣のペアの頻度をインクリメント
                if i+w < a.len()-1 && get_rt(&a, i+w) != 0 && !o {
                    //{{{
                    let rt_b: (usize, usize) = get_bg(&a, i);
                    if h.contains_key(&rt_b) {
                        let mut rt_r: &mut Rec = &mut **h.get(&rt_b).unwrap();
                        a[rt_r.loc].1 = Some(i);
                        a[i].2 = Some(rt_r.loc);
                        rt_r.loc = i;
                        out_rec(&mut q, &mut rt_r);
                        rt_r.freq += 1;
                        in_rec(&mut q, &mut rt_r);
                    }
                    else {
                        let mut new_r = Box::new(Rec {loc: i, freq: 1, prev: None, next: None});
                        in_rec(&mut q, &mut new_r);
                        let x: *mut Rec = Box::into_raw(new_r);
                        h.insert(rt_b, x);
                        a[i].2 = None;
                    }
                    //}}}
                }

                // 左隣のペアの頻度をインクリメント
                let mut pair_overlap = false;
                if i > 0 {
                    //{{{
                    let lt_i = get_lt(&a, i);
                    o = match a[i].1 {Some(x) => if x == lt_i {true} else {false}, None => false};
                    if o && get_bg(&a, lt_i) == get_bg(&a, i) {pair_overlap = true;}
                    let lt_b: (usize, usize) = get_bg(&a, lt_i);
                    if h.contains_key(&lt_b) {
                        let mut lt_r: &mut Rec = &mut **h.get(&lt_b).unwrap();
                        a[lt_r.loc].1 = Some(lt_i);
                        if !o {a[lt_i].1 = None;}
                        a[lt_i].2 = Some(lt_r.loc);
                        out_rec(&mut q, &mut lt_r);
                        lt_r.freq += 1;
                        lt_r.loc = lt_i;
                        in_rec(&mut q, &mut lt_r);
                    }
                    else {
                        let mut new_r = Box::new(Rec {loc: lt_i, freq: 1, prev: None, next: None});
                        in_rec(&mut q, &mut new_r);
                        let x: *mut Rec = Box::into_raw(new_r);
                        h.insert(lt_b, x);
                        if !o {a[lt_i].1 = None;}
                        a[lt_i].2 = None;
                    }
                    //}}}
                }

                if a[i].1 == None {break;}
                let ii = i;
                i = a[i].1.unwrap();
                if !pair_overlap {a[ii].1 = None;}
            //}}}
            }

            v += 1;
            h.remove(&b);
        }
    }

    let mut s: Vec<usize> = Vec::new();
    for c in &a {match (*c).0 {Some(x) => s.push(x), None => ()}}

    let end = start.elapsed();
    println!("[Result]");
    println!("alphabet size   : {:?}", z.len());
    println!("rule size       : {:?}", g.len());
    let mut dsize = 0;
    for mrl in &g {
        dsize += (*mrl).len();
    }
    println!("dictionary size : {:?}", dsize);
    println!("sequence length : {:?}", s.len());
    println!("total size      : {:?}", dsize + s.len());
    println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);

    if matches.is_present("print") {
        println!("\n[Grammar detail]");
        println!("alphabet   :\n {:?}", z);
        println!("dictionary :\n {:?}", g);
        println!("sequence   :\n {:?}", s);
    }

    // decompression test
    // {{{
    // let mut u: Vec<char> = Vec::new();
    // fn drv(i: usize, z: &Vec<char>, g: &Vec<Vec<usize>>, u: &mut Vec<char>) -> () {
    //     if i < z.len() {
    //         u.push(z[i]);
    //     }
    //     else {
    //         let bg = &g[i-z.len()];
    //         for x in bg {
    //             drv(*x, z, g, u);
    //         }
    //     }
    // }
    // for c in &s {
    //     drv(*c, &z, &g, &mut u);
    // }
    // // println!("{:?}", u);
    // let mut f = BufWriter::new(File::create(env::args().nth(2).unwrap()+".dcp").unwrap());
    // f.write(u.iter().collect::<String>().as_bytes()).unwrap();
    // }}}

    //}}}

}
