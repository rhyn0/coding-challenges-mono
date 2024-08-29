use clap::{Args, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, short, action = clap::ArgAction::Count)]
    pub debug: u8,

    /// When an option is specified, wc only reports the information requested by that option.
    /// The order of output always takes the form of line, word, byte, and file name.
    /// The default action is equivalent to specifying the -c, -l and -w options.
    #[command(flatten)]
    output: OutputOptions,

    /// Files to read from.
    pub files: Vec<String>,
}

#[derive(Args, Debug, Clone, Copy)]
#[group(required = false, multiple = true)]
#[allow(clippy::struct_excessive_bools)]
pub struct OutputOptions {
    ///  The number of bytes in each input file is written to the standard output.
    #[arg(short = 'c')]
    pub byte_count: bool,

    ///  The number of lines in each input file is written to the standard output.
    #[arg(short = 'l')]
    pub line_count: bool,

    /// The number of words in each input file is written to the standard output.
    #[arg(short = 'w')]
    pub word_count: bool,

    /// The number of characters in each input file is written to the standard output.
    /// If the current locale does not support multibyte characters, this is equivalent to the -c option.
    /// This will cancel out any prior usage of the -c option.
    #[arg(short = 'm')]
    pub character_count: bool,
}

impl Default for OutputOptions {
    fn default() -> Self {
        Self {
            byte_count: true,
            line_count: true,
            word_count: true,
            character_count: false,
        }
    }
}

impl Cli {
    pub fn get_output_settings(&self) -> OutputOptions {
        if self.output.byte_count
            || self.output.line_count
            || self.output.word_count
            || self.output.character_count
        {
            if self.output.character_count && self.output.byte_count {
                // if -m flag is specified, ignore character count flag
                OutputOptions {
                    byte_count: false,
                    ..self.output
                }
            } else {
                self.output
            }
        } else {
            OutputOptions::default()
        }
    }
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
    fn test_output_default_false() {
        let cli = Cli::try_parse_from("oxwc".split(" "));
        assert!(cli.is_ok());
        let result = cli.unwrap();
        assert_eq!(result.debug, 0);
        assert_eq!(result.output.byte_count, false);
        assert_eq!(result.output.word_count, false);
        assert_eq!(result.output.byte_count, false);
    }
    #[test]
    fn test_default_output_options() {
        let cli = Cli::try_parse_from("oxwc".split(" "));
        assert!(cli.is_ok());
        let result = cli.unwrap();
        let output_options = result.get_output_settings();
        assert_eq!(output_options.line_count, true);
        assert_eq!(output_options.word_count, true);
        assert_eq!(output_options.byte_count, true);
    }
    #[test]
    fn test_byte_group() {
        let cli = Cli::try_parse_from("oxwc -c -m".split(" "));
        assert!(cli.is_ok());
        let result = cli.unwrap();
        let output_options = result.get_output_settings();
        assert_eq!(output_options.byte_count, false);
        assert_eq!(output_options.character_count, true);
    }
}
