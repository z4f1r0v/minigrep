[![build](https://github.com/z4f1r0v/picogrep/actions/workflows/main.yml/badge.svg)](https://github.com/z4f1r0v/picogrep/actions/workflows/main.yml)
# picogrep

Yet another implementation of the [Rust book tutorial](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html)
for building a `grep` clone.

I'm going through `man grep` and choosing flags with increasing difficulty to expand my Rust skills. So far we have:
```sh
$ ./target/debug/picogrep -h
A mininal version of grep

Usage: picogrep [OPTIONS] <QUERY> <FILENAME>

Arguments:
  <QUERY>     The string to search for
  <FILENAME>  The file to search

Options:
  -c, --count             Return the amount matches
  -i, --case-insensitive  Perform a case insensitive search
  -h, --help              Print help
```