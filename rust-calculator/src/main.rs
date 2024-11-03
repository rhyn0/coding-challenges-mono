use clap::Parser;
use env_logger::Builder;

use log::{debug, LevelFilter};
use postfix::PostExpression;

mod args;
mod postfix;
mod tokens;

fn main() {
    let cli = args::Cli::parse();
    match cli.debug {
        0 => Builder::new().filter_level(LevelFilter::Warn).init(),
        1 => Builder::new().filter_level(LevelFilter::Info).init(),
        2 => Builder::new().filter_level(LevelFilter::Debug).init(),
        3.. => Builder::new().filter_level(LevelFilter::max()).init(),
    }

    debug!("staring evaluation of input '{}'", cli.math_expression);
    let eq = cli
        .math_expression
        .parse::<tokens::Expression>()
        .expect("Valid equation");
    debug!("Valid equation given - {:?}", eq);
    let postfix = PostExpression::from_infix(eq);
    println!("Result: {}", postfix.eval());
}
