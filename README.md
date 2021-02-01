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

Splits a SDF package into a n number of smaller packages.

```
USAGE:
    sdsplit path/to/package.sdf intOfPackages path/to/output/dir
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
    -i, --input <INPUT>                Sets input. Can be file or directory.
    -o, --output <OUTPUT>              Sets output directory. Must not be . as the output filename is the same as the input filename.
    -p, --pattern <PATTERN>            SDF field to write out [default: SCORE]
```