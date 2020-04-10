# Pipeviewer

This is a project that I'm building while I'm learning Rust.
Its purpose is to read something from an input and store in a buffer. It currently allows the following inputs:
- File
- If no file input is specified `stdin` will be the fallback

And then the CLI writes the buffer out to an output source. It can be:
- File
- If no file is specified it falls back to `stdout`

```
USAGE:
    pipeviewer [FLAGS] [OPTIONS] [infile]

FLAGS:
    -h, --help       Prints help information
    -s, --silent
    -V, --version    Prints version information

OPTIONS:
    -o, --outfile <outfile>    Write output to a file instead of stdout

ARGS:
    <infile>    Read from a file instead of stdin
```
