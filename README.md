# Coding Challenges by Ryan

Monorepo of my attempts at doing [Coding Challenges](https://codingchallenges.fyi/). Most attempts will be written in Rust as my language of choice.

## Code Quality

Using [pre-commit](https://pre-commit.com) I enforce some code quality on code change.

Checks:
    - Formatting - using rustfmt
    - Linting - using clippy

## Workspace

If you are like me, somethings about Rust and Cargo in general might not be burned into your brain. This is a Cargo Workspace. It means that the collection of projects in this folder can all use the same package resolution provided by being included in the workspace.

A lot of the common commands for building a singular package still apply - i.e. `cargo test` and `cargo build`. **BUT** these commands will run them for *each* member of the workspace - which may or may not be what is intended. Now you know.

If you only want to run that command against one package, you can use a `-p pkgid` on those commands. Simply put the `pkgid` is the value in `Cargo.toml` at `package.name`. To make this easier to remember, I try to name the folders of each the same, which is also what `cargo new` tends to do also.

> Run `cargo help pkgid` to learn more about `pkgid`.

### Adding Dependencies

While easiest to still change directory into the specific project, you can add dependencies to a project by doing `cargo add <DEP> -p <pkgid>`. E.g. `cargo add clap -p rust-wc`.

### Run Tests

Run all tests - `cargo test` in this directory `coding-challenges-mono` or whatever you have it named.

Run tests for one package `cargo test -p rust-wc`.
