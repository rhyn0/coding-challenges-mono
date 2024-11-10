# Simple Calculator in Rust

Allows a person to evaluate simple integer math operations via CLI.

**TODOS**
_In no particular order_
- [ ] Get floating point to work
- [ ] Handle unquoted expressions, e.g. `oxc 3 + 4` should work

## Downloading

### Manual Method

1. Have `cargo` and `rustc` installed. See the [Rust Book](https://doc.rust-lang.org/cargo/getting-started/installation.html).
1. Clone this repository. `git clone https://github.com/rhyn0/coding-challenges-mono.git`
1. `cd` into this folder (`coding-challenge-mono/rust-calculator`).
1. Build the release binary `cargo build -r`.

### From GitHub Releases

1. Download the binary from the [releases](https://github.com/rhyn0/coding-challenges-mono/releases)

## Usage

```plaintext
Usage: oxc [OPTIONS] <MATH_EXPRESSION>

Arguments:
  <MATH_EXPRESSION>  Infix calculation to compute

Options:
  -d, --debug...
  -h, --help      Print help
  -V, --version   Print version
```

> Make sure to quote the expression.

Example: `oxc '3 * 4'`
