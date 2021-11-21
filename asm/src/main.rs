use std::path::PathBuf;
use std::process;

use asm::Assembler;
use clap::{crate_authors, crate_version, Parser, ValueHint};
use env_logger as logger;
use log::error;

fn main() {
    // Initialize logger
    logger::Builder::new()
        .default_format()
        .format_indent(Some(12))
        .format_timestamp(None)
        .parse_default_env()
        .init();
    // Parse opts
    let opts = Opts::parse();

    // Instantiate an assembler
    let mut a = Assembler::new();
    // Source each input file
    for file in &opts.srcs {
        a.src(file).unwrap_or_else(|err| {
            eprintln!("{}: `{}`", err, file.display());
            process::exit(1);
        });
    }
    // Produce an assembled output
    a.asm().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    // Write output file
    a.out(&opts.out).unwrap_or_else(|err| {
        error!("{}: `{}`", err, &opts.out.display());
        process::exit(1);
    });
}

/// Assembler for the KAP-16 processor.
#[derive(Debug, Parser)]
#[clap(author = crate_authors!())]
#[clap(version = crate_version!())]
struct Opts {
    /// Input source file
    #[clap(parse(from_os_str))]
    #[clap(required = true)]
    #[clap(min_values = 1)]
    #[clap(value_hint = ValueHint::FilePath)]
    srcs: Vec<PathBuf>,

    /// Output binary file
    #[clap(short)]
    #[clap(long)]
    #[clap(parse(from_os_str))]
    #[clap(default_value = "a.out")]
    #[clap(value_hint = ValueHint::FilePath)]
    out: PathBuf,

    /// Use verbose output (-v, -vv, -vvv, etc.)
    #[clap(short)]
    #[clap(long)]
    #[clap(parse(from_occurrences))]
    verbose: u8,
}
