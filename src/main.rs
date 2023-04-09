use clap::Parser;
use commache::cli;
use commache::config;
use tracing::debug;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() {
    // set up some sort of logability while I figure things out
    // TODO tidy away somewhere
    color_backtrace::install();
    let file_appender = tracing_appender::rolling::never(
        std::env::temp_dir(),
        format!("{}.log", std::env!("CARGO_PKG_NAME")),
    );
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let filter = EnvFilter::from_default_env();
    let file_appender = fmt::layer().with_writer(non_blocking);
    tracing_subscriber::registry()
        .with(filter)
        .with(file_appender)
        .init();

    debug!("config: {:?}", config::get());

    let args = cli::Args::parse();
    commache::main(args);
}
