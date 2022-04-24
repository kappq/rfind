# rfind
`rfind` is a simple clone of the `find` command written in Rust.

## Getting Started
Install the program with `cargo`:
```
cargo install rfind
```

## Usage
The usage is very similar to the `find` command, but a lot simpler. Type `rfind --help` to see an help message.

```
rfind 0.2.0
Kappa <f.cappetti.05@gmail.com>
A simple clone of the `find` command

USAGE:
    rfind [OPTIONS] [starting-point]...

ARGS:
    <starting-point>...    The starting-point of the program [default: .]

OPTIONS:
    -h, --help           Print help information
    -n, --name <name>    The regex to match
    -t, --type <type>    The type of the file [possible values: f, d, l]
    -V, --version        Print version information
```

## To-Do
- [ ] Testing;
- [ ] Improve error handling;
- [x] Improve CLI interface and argument parsing;
- [x] Multiple starting points;
- [ ] Options for size, modification date, permission and owner;
- [ ] Delete files.
