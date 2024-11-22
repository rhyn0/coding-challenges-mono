mod list;
use env_logger::Builder;
use log::LevelFilter;

fn main() {
    Builder::new().filter_level(LevelFilter::Error).init();
}
