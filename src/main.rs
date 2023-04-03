use clap::Parser;
use commache::cli;
use commache::config;
use tracing::debug;

fn main() {
    color_backtrace::install();
    debug!("config: {:?}", config::get());

    let args = cli::Args::parse();
    commache::main(args);
}
