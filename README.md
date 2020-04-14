# SDF-filter
This is a CLI tool for filtering docking results made by RxDock or similar software.

## Build
```
cd /path/to/SDF-filter-master
mkdir build
pub get
dart2native bin/main.dart -o build/sdfilter
```

## Manual
### Usage
```
sdfilter -i /path/to/input(.sdf) -o /path/to/output.sdf -l/e/g -f <double> [-p <SCORE>]
```
### Options
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
