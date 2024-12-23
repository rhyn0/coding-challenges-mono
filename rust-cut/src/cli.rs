use std::str::FromStr;

use clap::{Args, Parser};

use crate::range::CutList;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub selectors: Selectors,

    /// use DELIM instead of TAB for field delimiter
    #[arg(long, short, default_value_t = '\t')]
    pub delimiter: char,

    /// complement the set of selected bytes, characters or fields
    #[arg(long)]
    pub complement: bool,

    /// do not print lines not containing delimiters
    #[arg(short = 's', long)]
    pub only_delimited: bool,

    /// use STRING as the output delimiter the default is to use
    /// the input delimiter
    /// Only has an effect on selecting fields - not an error to specify otherwise.
    #[arg(long)]
    pub output_delimiter: Option<String>,

    /// Files to read from.
    pub files: Vec<String>,

    /// line delimiter is NUL, not newline
    #[arg(short, long)]
    pub zero_terminated: bool,

    #[arg(long, short, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Selectors {
    /// Select only these bytes.
    #[arg(long, short, value_parser = CutList::from_str)]
    pub bytes: Option<CutList>,
    /// select only these characters
    #[arg(long, short, value_parser = CutList::from_str)]
    pub characters: Option<CutList>,
    ///  select only these fields;  also print any line that
    /// contains no delimiter character, unless the -s option is
    /// specified
    #[arg(long, short, value_parser = CutList::from_str)]
    pub fields: Option<CutList>,
}

#[cfg(test)]
mod tests {

    use std::error::Error;

    use clap::CommandFactory;

    use super::*;
    use crate::range::cut::{self, CutRange};

    #[test]
    fn test_cli() {
        Cli::command().debug_assert();
    }
    #[test]
    fn test_parse_cut_range_single() {
        let args = Cli::parse_from("oxcut -b 1 -".split_whitespace());
        assert_eq!(args.files.len(), 1);
        let byte_selector = args.selectors.bytes.unwrap();
        assert_eq!(byte_selector, CutList::new(vec![CutRange::from(1)]));
        assert_eq!(
            byte_selector,
            CutList::new(vec![cut::CutRange::from(1usize)])
        );
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
    #[test]
    fn test_parse_space_ranges() {
        let res = Cli::try_parse_from(vec!["oxcut", "-b1 2", "-"]);
        let bytes_selector = res.unwrap().selectors.bytes.unwrap();
        assert_eq!(
            bytes_selector,
            CutList::new(vec![CutRange::from(1), CutRange::from(2)])
        );
    }
    #[test]
    fn test_parse_comma_ranges() {
        let res = Cli::try_parse_from(vec!["oxcut", "-b1,2", "-"]);
        let bytes_selector = res.unwrap().selectors.bytes.unwrap();
        assert_eq!(
            bytes_selector,
            CutList::new(vec![CutRange::from(1), CutRange::from(2)])
        );
    }
}
