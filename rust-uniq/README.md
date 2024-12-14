# GNU Uniq in Rust

Remove repeated adjacent lines from input file.

Based on the GNU core util - [`uniq`](https://www.gnu.org/software/coreutils/manual/html_node/uniq-invocation.html#uniq-invocation).

## TODOs

In no particular order:

- [ ] need to handle preservation of line ending sequence. Technically part of the line and needs to be compared.
- [ ] Small divergence against GNU uniq, when final line does not end with a newline.


## Downloading

### Manual Method

1. Have `cargo` and `rustc` installed. See the [Rust Book](https://doc.rust-lang.org/cargo/getting-started/installation.html).
    - This was built and tested with `rustc 1.79.0`
1. Clone this repository. `git clone https://github.com/rhyn0/coding-challenges-mono.git`
1. `cd` into this folder (`coding-challenge-mono/rust-uniq`).
1. Build the release binary `cargo build -r`.

### From GitHub Releases

1. Download the binary from the [releases](https://github.com/rhyn0/coding-challenges-mono/releases)

## Usage

TODO: should closely match the GNU one.



## Example:

```shell
echo 'hi
hi
hi' | oxuniq
```
