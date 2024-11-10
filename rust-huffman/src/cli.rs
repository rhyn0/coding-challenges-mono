use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, short, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Commands {
    /// encode the given data file
    Encode {
        /// file to encode
        input_file: String,
        /// file path to write encoded to
        output_file: String,
    },
    /// Decode given files
    Decode {
        /// file to decode
        input_file: String,
        /// file path to write decoded to
        output_file: String,
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
            "oxhuff decode les-mis.encoded.txt static/result/les-mis.txt".split_ascii_whitespace(),
        );
        assert_eq!(
            args.command,
            Commands::Decode {
                input_file: "les-mis.encoded.txt".into(),
                output_file: "static/les-mis.txt".into(),
            }
        );
    }

    #[test]
    fn test_example_encode() {
        let args = Cli::parse_from(
            "oxhuff encode static/les-mis.txt les-mis.encoded.txt".split_ascii_whitespace(),
        );
        assert_eq!(
            args.command,
            Commands::Encode {
                input_file: "static/les-mis.txt".into(),
                output_file: "les-mis.encoded.txt".into()
            }
        );
    }
}
