# stringc

Simple translations manager and generator. It can generate localizable files for iOS (Localizable.strings) and for Android (strings.xml).  
It can also generate Excel (xlsx) files with keys and strings to translate then filled Excel file can be imported.  

All data is stored in JSON file.

## Usage

```text
.\stringc.exe -h
stringc 0.1.0
Paweł Wróblewski
Simple strings generator and manager.

USAGE:
    stringc.exe [FLAGS] [OPTIONS] <data> --project <project>

ARGS:
    <data>

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    Verbose mode
    -V, --version    Prints version information

OPTIONS:
    -o, --out <output>         Comma separated strings output for iOS and/or Android targets. ex.
                               "-o and,ios"
    -p, --project <project>    Selects project to work on
```