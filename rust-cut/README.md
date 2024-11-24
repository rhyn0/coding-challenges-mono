# GNU Cut in Rust

Cut out selected portions of each line.

Based on the GNU core util - [`cut`](https://www.gnu.org/software/coreutils/manual/html_node/The-cut-command.html).


## Downloading

### Manual Method

1. Have `cargo` and `rustc` installed. See the [Rust Book](https://doc.rust-lang.org/cargo/getting-started/installation.html).
    - This was built and tested with `rustc 1.79.0`
1. Clone this repository. `git clone https://github.com/rhyn0/coding-challenges-mono.git`
1. `cd` into this folder (`coding-challenge-mono/rust-cut`).
1. Build the release binary `cargo build -r`.

### From GitHub Releases

1. Download the binary from the [releases](https://github.com/rhyn0/coding-challenges-mono/releases)

## Usage

```plaintext
Usage: oxcut [OPTIONS] <--bytes <BYTES>|--characters <CHARACTERS>|--fields <FIELDS>> [FILES]...

Arguments:
  [FILES]...  Files to read from

Options:
  -b, --bytes <BYTES>
          Select only these bytes
  -c, --characters <CHARACTERS>
          select only these characters
  -f, --fields <FIELDS>
          select only these fields;  also print any line that contains no delimiter character, unless the -s option is specified
  -d, --delimiter <DELIMITER>
          use DELIM instead of TAB for field delimiter [default: "\t"]
      --complement
          complement the set of selected bytes, characters or fields
  -s, --only-delimited
          do not print lines not containing delimiters
      --output-delimiter <OUTPUT_DELIMITER>
          use STRING as the output delimiter the default is to use the input delimiter Only has an effect on selecting fields - not an error to specify otherwise
  -z, --zero-terminated
          line delimiter is NUL, not newline
  -v, --verbose...

  -h, --help
          Print help
  -V, --version
          Print version
```


Example: `echo 'hi:hello | oxcut -d: -f1`
