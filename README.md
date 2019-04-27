# MR-RePair Grammar Compressor

### Description

This is an implementation of MR-RePair, which is a variant of RePair (about RePair, see https://github.com/izflare/RePair).  
MR-RePair is proposed in

> I. Furuya, T. Takagi, Y. Nakashima, S. Inenaga, H. Bannai and T. Kida: _MR-RePair: Grammar Compression based on Maximal Repeats._ DCC 2019, pp.508-517.

Note that this implementation is not used in experiments of above paper.  
This code constructs only a grammar.
Encoding process is unimplemented yet.

### Download

```
git clone https://github.com/izflare/MR-RePair.git
```

### Compile

This code has been tested under linux compiling with rust (cargo) ver 1.33.0.  
After download the repository, 

```
cd MR-RePair
cargo build --release
```

### Run

After compiling,

```
cd target/release
./mrrp --input <input> [--print]
```

`<input>` is your input text data file.  
Size of constructed grammar and elapsed time for running will be displayed.  
If you execute with `--print` option, constructed grammar will also be displayed.

