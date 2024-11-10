use clap::Parser;
use env_logger::Builder;

use log::{debug, error, LevelFilter};
use postfix::PostExpression;

mod args;
mod postfix;
mod tokens;

fn main() {
    let cli = args::Cli::parse();
    match cli.debug {
        0 => Builder::new().filter_level(LevelFilter::Error).init(),
        1 => Builder::new().filter_level(LevelFilter::Warn).init(),
        2 => Builder::new().filter_level(LevelFilter::Info).init(),
        3.. => Builder::new().filter_level(LevelFilter::max()).init(),
    }

    debug!("staring evaluation of input '{}'", cli.math_expression);
    let eq = cli
        .math_expression
        .parse::<tokens::Expression>()
        .expect("Valid equation");
    debug!("Valid equation given - {:?}", eq);
    let postfix = match PostExpression::try_from(eq) {
        Ok(x) => x,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };
    println!("Result: {}", postfix.eval());
}
