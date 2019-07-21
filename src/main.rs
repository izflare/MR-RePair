#[macro_use]
extern crate clap;
extern crate bit_vec;

use clap::{App, Arg, ArgGroup, AppSettings};
use std::io::{prelude::*, BufReader, BufWriter};
use std::fs::File;
use std::time::Instant;
use bit_vec::BitVec;
use mrrp::module::encode;
use mrrp::module::compress;
use mrrp::module::{cfg::*};

fn main() {

    // args
    let app = App::new("MR-RePair")
        //{{{
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::DeriveDisplayOrder)
        .args_from_usage("-c 'Compression mode'
                         -d 'Decompression mode'")
        .group(ArgGroup::with_name("mode").args(&["c", "d"]).required(true))
        .arg(Arg::from_usage("-i --input [FILE] 'Input sourse file'").required(true))
        .arg(Arg::from_usage("-m --minfreq [INTEGER] 'Sets minimum frequency of pairing operation'").default_value("2"))
        .arg(Arg::from_usage("-e --encode [MODE] 'Sets encoding mode'")
             .possible_values(&["32bit", "FBLE", "Huffman_coding", "POPPT+IBLE", "POPPT+PGE"])
             .default_value("POPPT+PGE"))
        .arg(Arg::from_usage("-b --blocksize [INTEGER] 'Sets block size of PGE'").default_value("8"))
        .arg(Arg::from_usage("-p --print 'Prints the detail of constructed grammar'"))
        .arg(Arg::from_usage("--debug 'Debug mode'"));
        //}}}
    let matches = app.get_matches();

    // read
    let mut s: Vec<u8> = Vec::new();
    let mut f = BufReader::new(File::open(&matches.value_of("input").unwrap()).expect("file not found"));
    f.read_to_end(&mut s).expect("Unable to read");

    // compression
    if matches.is_present("c") {
        let start = Instant::now();

        let minfreq = 
                std::cmp::max(2, match matches.value_of("minfreq") {Some(x) => (*x).parse::<usize>().unwrap(), None => 2,});
        let blocksize = match matches.value_of("blocksize") {Some(x) => (*x).parse::<u32>().unwrap(), None => 8,};
        let mode = matches.value_of("encode").unwrap();

        let mut g: Grammar = Grammar::new();
        compress::compress(&s, &mut g, minfreq);

        let end = start.elapsed();
        println!("[Result: grammar construction]");
        //{{{
        println!("Alphabet size     : {:?}", g.terminal.len());
        println!("Rule number       : {:?}", g.rule.len());
        println!("Dictionary size   : {:?}", g.rule.iter().fold(0, |sum, x| sum + x.len()));
        println!("Sequence length   : {:?}", g.sequence.len());
        println!("Total size        : {:?}", g.terminal.len() + g.rule.iter().fold(0, |sum, x| sum + x.len()) + g.sequence.len());
        println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);
        //}}}

        // encode
        let mut bv: BitVec = BitVec::new();
        encode::encode(&g, mode, &mut bv, blocksize);

        // write
        let mut f = BufWriter::new(File::create(matches.value_of("input").unwrap().to_owned()+".mrrp").unwrap());
        f.write(&bv.to_bytes()).unwrap();

        println!("[Result: compression]");
        println!("Input data        : {:?} [bytes]", s.len());
        println!("Compressed data   : {:?} [bytes]", bv.len() / 8 + if bv.len() % 8 > 0 {1} else {0});
        println!("Compression ratio : {:.3} [%]", 100.0 * bv.len() as f64 / 8.0 / s.len() as f64);
        if matches.is_present("print") {
            println!("\n[Grammar detail]");
            println!("Alphabet   :\n {:?}", g.terminal);
            println!("Dictionary :\n {:?}", g.rule);
            println!("Sequence   :\n {:?}", g.sequence);
        }

        if matches.is_present("debug") {
        //{{{
            println!("[Debug mode]");

            let mut debug_u: Vec<u8> = Vec::new();
            g.derive(&mut debug_u);
            assert!(s == debug_u, "The constructed grammar is incorrect.");
            println!("Grammar       : OK");

            let mut debug_g: Grammar = Grammar::new();
            encode::decode(&bv, &mut debug_g);
            debug_u = Vec::new();
            debug_g.derive(&mut debug_u);
            assert!(s == debug_u, "The encoding method is bugged.");
            println!("Encoding      : OK");
        //}}}
        }


    }

    // decompression
    else if matches.is_present("d") {
        let start = Instant::now();

        let bv: BitVec = BitVec::from_bytes(&s);
        let mut u: Vec<u8> = Vec::new();
        compress::decompress(&bv, &mut u);

        let end = start.elapsed();
        println!("[Result: decompression]");
        println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);

        // write
        let mut f = BufWriter::new(File::create(matches.value_of("input").unwrap().to_owned()+".dcp").unwrap());
        f.write(&u).unwrap();
    }
    else {
        panic!("mdoe error");
    }

}
