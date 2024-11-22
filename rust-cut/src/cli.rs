use std::str::FromStr;

use clap::Parser;

use crate::list;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Select only these bytes.
    #[arg(long, short, value_parser = list::CutRange::from_str)]
    pub bytes: Vec<list::CutRange>,

    /// use DELIM instead of TAB for field delimiter
    #[arg(long, short, default_value_t = '\t')]
    pub delimiter: char,

    ///  select only these fields;  also print any line that
    /// contains no delimiter character, unless the -s option is
    /// specified
    #[arg(long, short, value_parser = list::CutRange::from_str)]
    pub fields: Vec<list::CutRange>,

    /// Files to read from.
    pub files: Vec<String>,

    #[arg(long, short, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[cfg(test)]
mod tests {

    use std::error::Error;

    use clap::CommandFactory;

    use super::*;

    #[test]
    fn test_cli() {
        Cli::command().debug_assert();
    }
    #[test]
    fn test_parse_cut_range_single() {
        let args = Cli::parse_from("oxcut -b 1 -".split_whitespace());
        assert_eq!(args.files.len(), 1);
        assert_eq!(args.bytes.len(), 1);
        assert_eq!(args.bytes[0], list::CutRange::from(1usize));
    }
    #[test]
    fn test_parse_cut_range_single_illegal() {
        let res = Cli::try_parse_from("oxcut -b -".split_whitespace());
        assert!(
            res.is_err_and(|e| e.kind() == clap::error::ErrorKind::ValueValidation
                && e.source()
                    .unwrap()
                    .to_string()
                    .contains("illegal list value"))
        );
    }
    #[test]
    fn test_parse_cut_range_single_zero() {
        let res = Cli::try_parse_from("oxcut -b 0".split_whitespace());
        assert!(
            res.is_err_and(|e| e.kind() == clap::error::ErrorKind::ValueValidation
                && e.source()
                    .unwrap()
                    .to_string()
                    .contains("values may not include zero"))
        );
    }
    // TODO: this functionality is necessary
    #[test]
    #[should_panic]
    fn test_parse_space_ranges() {
        let res = Cli::try_parse_from(vec!["oxcut", "-b\"1 2\"", "-"]);
        assert!(res.is_ok_and(|cli| cli.bytes.len() == 2));
    }
    #[test]
    #[should_panic]
    fn test_parse_comma_ranges() {
        let res = Cli::try_parse_from(vec!["oxcut", "-b1,2", "-"]);
        assert!(res.is_ok_and(|cli| cli.bytes.len() == 2));
    }
}
