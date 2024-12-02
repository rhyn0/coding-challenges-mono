use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
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
