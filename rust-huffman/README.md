# Huffman Compression in Rust

Compress files and then decompress.

**TODOS**
_In no particular order_
- [ ] Trait and struct based instead of functional

## Downloading

### Manual Method

1. Have `cargo` and `rustc` installed. See the [Rust Book](https://doc.rust-lang.org/cargo/getting-started/installation.html).
1. Clone this repository. `git clone https://github.com/rhyn0/coding-challenges-mono.git`
1. `cd` into this folder (`coding-challenge-mono/rust-huffman`).
1. Build the release binary `cargo build -r`.

### From GitHub Releases

1. Download the binary from the [releases](https://github.com/rhyn0/coding-challenges-mono/releases)

## Usage

```plaintext
Usage: oxhuff [OPTIONS] <COMMAND>

Commands:
  encode  encode the given data file
  decode  Decode given files
  help    Print this message or the help of the given subcommand(s)

Options:
  -d, --debug...
  -h, --help      Print help
  -V, --version   Print version

## Encode Help
encode the given data file

Usage: oxhuff encode <INPUT_FILE> <OUTPUT_FILE>

Arguments:
  <INPUT_FILE>   file to encode
  <OUTPUT_FILE>  file path to write encoded to

Options:
  -h, --help  Print help

## Decode Help
Decode given files

Usage: oxhuff decode <INPUT_FILE> <OUTPUT_FILE>

Arguments:
  <INPUT_FILE>   file to decode
  <OUTPUT_FILE>  file path to write decoded to

Options:
  -h, --help  Print help
```


Example: `oxhuff encode README.md compress-README`
