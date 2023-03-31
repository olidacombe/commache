use clap::Parser;
use commache::cli;
use commache::config;

fn main() {
    color_backtrace::install();
    dbg!(config::get());
    // let args: Vec<String> = std::env::args().collect();
    // let arrgs: Vec<&str> = args.iter().map(|a| a.as_str()).collect();
    // dbg!(&arrgs);

    let args = cli::Args::parse();

    commache::main(&args.get());
}
