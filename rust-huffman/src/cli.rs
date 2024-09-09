use std::ffi::OsString;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, short, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[arg(long, short = 'f')]
    pub header_file: OsString,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Commands {
    /// encode the given data file
    Encode {
        /// file to encode
        file: OsString,
    },
    /// Decode given files
    Decode {
        /// file to decode
        file: OsString,
    },
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn test_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn test_example_decode() {
        let args = Cli::parse_from(
            "oxhuff --header-file ./test.hout decode static/les-mis.txt".split_ascii_whitespace(),
        );
        assert_eq!(args.header_file, "./test.hout");
        assert_eq!(
            args.command,
            Commands::Decode {
                file: "static/les-mis.txt".into()
            }
        );
    }

    #[test]
    fn test_example_encode() {
        let args = Cli::parse_from(
            "oxhuff --header-file ./test.hout encode static/les-mis.txt".split_ascii_whitespace(),
        );
        assert_eq!(args.header_file, "./test.hout");
        assert_eq!(
            args.command,
            Commands::Encode {
                file: "static/les-mis.txt".into()
            }
        );
    }
}
