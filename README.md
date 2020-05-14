# SDF Tools

A collection of CLI tools for editing and filtering docking results made by RxDock or similar software.

## Build

### Prerequisites

* [dart](https://dart.dev)

For ease of building we have included a script.

To build every tool run:

```
cd /path/to/SDF-filter-master
pub get
./build.sh all 
```

Windows users must substitute `./build.sh` with `build.bat`.

If you only need to build a specific tool (ex. sdfilter) substitute `all` for `<tool name>` (ex. `sdfilter`)

## Manual

### sdfilter

#### Usage

```
sdfilter -i /path/to/input(.sdf) -o /path/to/output.sdf -l/e/g -f <double> [-p <SCORE>]
```

#### Options

```
-i    --input               Input can be file or directory.
                            Absolute path must be specified.

-o    --output              Output must be sdf file
                            Absolute path must be specified.

Specify only one operand!
-l    --lt                  Less than operand.
-e    --eq                  Equal operand.
-g    --gt                  Greater than operand.

-f    --filter              Value to be compared to.

-p    --pattern             SDF field to check. Ex: <SCORE>
                            Must end with > character
                            Defaults to <SCORE>
```
