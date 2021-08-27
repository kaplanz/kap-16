use std::path::PathBuf;
use std::process;

use asm::Assembler;
use clap::{crate_authors, crate_version, Clap, ValueHint};
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
    let opt = Opt::parse();

    // Instantiate an assembler
    let mut a = Assembler::new();
    // Source each input file
    for file in &opt.srcs {
        a.source(file).unwrap_or_else(|err| {
            error!("`{}`: {}", file.display(), err);
            process::exit(1);
        });
    }
    // Produce an assembled output
    a.assemble();
}

/// Emulator for the KAP-16 processor.
#[derive(Clap, Debug)]
#[clap(author = crate_authors!())]
#[clap(version = crate_version!())]
struct Opt {
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
    #[clap(default_value = "a.rom")]
    #[clap(value_hint = ValueHint::FilePath)]
    out: PathBuf,

    /// Use verbose output (-v, -vv, -vvv, etc.)
    #[clap(short)]
    #[clap(long)]
    #[clap(parse(from_occurrences))]
    verbose: u8,
}
