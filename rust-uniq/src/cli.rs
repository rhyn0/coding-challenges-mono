use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Print the number of times each line occurred along with the line.
    #[arg(short, long, group = "output_type")]
    pub count: bool,
    /// Discard lines that are not repeated. When used by itself, this
    /// option causes uniq to print the first copy of each repeated line,
    /// and nothing else.
    #[arg(short = 'd', long, group = "selection")]
    pub repeated: bool,
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_assert() {
        Args::command().debug_assert();
    }
}
