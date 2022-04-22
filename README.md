# rfind
`rfind` is a simple clone of the `find` command written in Rust.

## Getting Started
Install the program with `cargo`:
```
cargo install rfind
```

## Usage
The usage is very similar to the `find` command, but a lot simpler.

### Syntax
```
rfind starting-point expression
```
where `starting-point` is a path to a directory and `expression` is a list of options.

### Options
There are only two options:
- `-name <regex>` matches the given regex against the file name;
- `-type f|d|l` matches the file type and it can be:
    - `f` matches a file;
    - `d` matches a directory;
    - `l` matches a symbolic-link.

## To-Do
- [ ] Multiple starting points;
- [ ] Improved CLI and argument parsing;
- [ ] Options for size, modification date, permission and owner;
- [ ] Delete files.