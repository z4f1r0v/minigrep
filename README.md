[![build](https://github.com/z4f1r0v/picogrep/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/z4f1r0v/picogrep/actions/workflows/main.yml)

# picogrep

Yet another implementation of the [Rust book tutorial](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html)
for building a `grep` clone.

I'm going through `man grep` and choosing flags with increasing difficulty to expand my Rust skills. So far we have:
```sh
$ ./target/debug/picogrep -h                   
picogrep 
picogrep

USAGE:
    picogrep [OPTIONS] [ARGS]

ARGS:
    <QUERY>       
    <FILENAME>    

OPTIONS:
    -c, --count          Return the amount matches
    -h, --help           Print help information
    -i, --ignore-case    Ignore case sensitivity

```