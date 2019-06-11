## MR-RePair

### Description

This is an implementation of MR-RePair, a variant of RePair.
This method is proposed in

> I. Furuya, T. Takagi, Y. Nakashima, S. Inenaga, H. Bannai and T. Kida: _MR-RePair: Grammar Compression based on Maximal Repeats._ DCC 2019, pp.508-517.

Note that, instead of this implementation, https://github.com/tkida/MR-Repair is used for the experiments of the paper.

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
    cd target/release
    ./target/release/mrrp [FLAGS] [OPTIONS] --input <input> <-c|-d>

FLAGS:
    -c               Compression mode
    -d               Decompression mode
    -h, --help       Prints help information
    -p, --print      Print the detail of constructed grammar
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>    Input sourse text file
    -m, --min <minfreq>    Set minimum frequency of pairing operation (default: 3)
```

