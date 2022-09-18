# Wikipedia to Wordlist
A small tool written in rust to create wordlists based on wikipedia articles.

![GitHub](https://img.shields.io/github/license/gkourgkoutas/w2w) ![Rustversion](https://img.shields.io/badge/rustc-1.63.0-red)
### Usage
```sh
w2w 0.1.3
CLI Tool to generate wordlists based on wikipedia articles

USAGE:
    w2w [OPTIONS] --output <output> --search <search>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --lang <lang>        Set the article language [default: en]
    -o, --output <output>    Outputfile
    -s, --search <search>    Search wikipedia by keyword
```
### Example
```sh
$ ./w2w -s "John Titor" -o ~/Path_to_outputfile/wordlist.txt
```

### Contribution
Feel free to contribute! If you find a bug, want to add a feature or something else: Submit a pull request.