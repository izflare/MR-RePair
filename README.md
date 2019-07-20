## MR-RePair

### Description

This is an implementation of MR-RePair, which is proposed in

> I. Furuya, T. Takagi, Y. Nakashima, S. Inenaga, H. Bannai and T. Kida: _MR-RePair: Grammar Compression based on Maximal Repeats._ DCC 2019, pp.508-517.

Note that this implementation is not used for the experiments of the paper
(see https://github.com/tkida/MR-Repair).

The constructed grammar is encoded by using post-order partial parse tree (POPPT) for now.

### Download

```
git clone https://github.com/izflare/MR-RePair.git
```

### Compile

This code has been tested under linux compiling with rust (cargo) ver 1.34.0.  

```
cd MR-RePair
cargo build --release
```

### Run

```
USAGE:
    ./target/release/mrrp [FLAGS] [OPTIONS] --input <input> <-c|-d>

FLAGS:
    -c               Compression mode
    -d               Decompression mode
    -p, --print      Prints the detail of constructed grammar
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>         Input sourse file
    -m, --minfreq <INTEGER>    Sets minimum frequency of pairing operation [default: 2]
    -e, --encode <MODE>        Sets encoding mode [default: POPPT]  
	                           [possible values: 32bit, FBLE, Huffman_coding, POPPT+IBLE, POPPT+PGE]
```

The command with `-c` flag produces the compressed file `<FILE>.mrrp`.  
The command with `-d` flag produces the decompressed file `<FILE>.dcp`.
