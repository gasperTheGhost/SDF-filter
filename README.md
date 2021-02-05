# SDF Tools

A collection of CLI tools for editing and filtering docking results made by CmDock or similar software.

## Build

### Prerequisites

* [Rust](https://www.rust-lang.org/)

To build every tool run:

```
cargo build --release
```

## Manual

### sdsplit

Splits SDF file into a specified number of equal files or files with an equal number of records.

```
USAGE:
    sdsplit [OPTIONS] --input <INPUT> --output <OUTPUT> 
    [--files <NUM OF PACKAGES>/--size <PACKAGE SIZE>]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <INPUT>              Sets input file. - reads from stdin.
    -f, --files <NUM OF PACKAGES>    Sets nr of packeges to split input into.
    -s, --size <PACKAGE SIZE>        Sets size of packaes to split input into.
    -o, --output <OUTPUT>            Sets output directory. [default: .]
    -n, --name <NAME>                Sets output file names. This is
                                     always suffixed with _# [default: temp]
    
    --files and --size are incompatible, but at least one must be specified
```

### sdreport

Reads through SDF records, extracts the data from a specified field (pattern) and exports it into (a) file(s) with each value in a new line.

```
USAGE:
    sdreport [FLAGS] [OPTIONS] --input <INPUT> --output <OUTPUT>

FLAGS:
    -h, --help         Prints help information
    -r, --recursive    Enables recursive directory crawling
    -V, --version      Prints version information

OPTIONS:
    -i, --input <INPUT>           Sets input. Can be file or directory.
                                  Reads from stdin if set to -
    -o, --output <OUTPUT>         Sets output directory. Must not be
                                  . as the output filename is the
                                  same as the input filename.
    -p, --pattern <PATTERN>...    SDF field to write out. [default: SCORE]
```

### sdcount

Returns number of SDF records in file or stdin.

```
USAGE:
    sdcount <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <INPUT>    Sets input file. If missing
               reads from stdin.
```