use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
    /// Print the number of times each line occurred along with the line.
    #[arg(short, long, group = "output_type")]
    pub count: bool,
    /// Discard lines that are not repeated. When used by itself, this
    /// option causes uniq to print the first copy of each repeated line,
    /// and nothing else.
    #[arg(short = 'd', long, group = "selection")]
    pub repeated: bool,
    /// ignore case seems like the best next one `-i`
    #[arg(short = 'i', long, group = "selection")]
    pub ignore_case: bool,
    /// Do not discard the second and subsequent repeated input lines,
    // but discard lines that are not repeated. This option is useful
    // mainly in conjunction with other options e.g., to ignore case or
    // to compare only selected fields. The optional delimit-method,
    // supported with the long form option, specifies how to delimit groups
    // of repeated lines, and must be one of the following:
    #[arg(
        short = 'D',
        long,
        value_enum,
        group = "selection",
        default_missing_value = "none",
        num_args=0..=1,
    )]
    pub all_repeated: Option<DelimitMethod>,
    /// Delimit items with a zero byte rather than a newline (ASCII LF).
    /// I.e., treat input as items separated by ASCII NUL and terminate
    /// output items with ASCII NUL. This option can be useful in conjunction
    /// with ‘perl -0’ or ‘find -print0’ and ‘xargs -0’ which do the same in
    /// order to reliably handle arbitrary file names (even those containing
    /// blanks or other special characters). With -z the newline character is
    /// treated as a field separator.
    #[arg(short, long, group = "selection")]
    pub zero_terminated: bool,
    /// Discard the last line that would be output for a
    /// repeated input group. When used by itself, this option
    /// causes uniq to print unique lines, and nothing else.
    #[arg(short, long, group = "selection")]
    pub unique: bool,
    #[arg(value_name = "input", group = "input")]
    pub input_file: Option<String>,
    #[arg(value_name = "output", requires = "input")]
    pub output_file: Option<String>,
    #[arg(long, short, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Ord, ValueEnum)]
pub enum DelimitMethod {
    /// Do not delimit groups of repeated lines. This is equivalent to --all-repeated (-D).
    None,
    /// Output a newline before each group of repeated lines.
    // With --zero-terminated (-z), use a zero byte (ASCII NUL)
    // instead of a newline as the delimiter.
    Prepend,
    /// Separate groups of repeated lines with a single newline.
    // This is the same as using ‘prepend’, except that no delimiter
    // is inserted before the first group, and hence may be better
    // suited for output direct to users. With --zero-terminated (-z),
    // use a zero byte (ASCII NUL) instead of a newline as the delimiter.
    Separate,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_assert() {
        Args::command().debug_assert();
    }
    #[test]
    fn test_default_all_repeated() {
        let args = Args::parse_from(&["uniq", "-D"]);
        assert_eq!(args.all_repeated, Some(DelimitMethod::None));
        let args = Args::parse_from(&["uniq", "--all-repeated"]);
        assert_eq!(args.all_repeated, Some(DelimitMethod::None));
    }
    #[test]
    fn test_specific_all_repeated() {
        for (string, expected) in &[
            ("none", DelimitMethod::None),
            ("prepend", DelimitMethod::Prepend),
            ("separate", DelimitMethod::Separate),
        ] {
            let args = Args::parse_from(&["uniq", "-D", string]);
            assert_eq!(args.all_repeated, Some(*expected));
            let args = Args::parse_from(&["uniq", "--all-repeated", string]);
            assert_eq!(args.all_repeated, Some(*expected));
        }
    }
}
