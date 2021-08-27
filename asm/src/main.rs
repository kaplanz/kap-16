use std::path::PathBuf;

use asm::Assembler;
use clap::{crate_authors, crate_version, Clap, ValueHint};
use env_logger as logger;

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
    let a = Assembler::new(&opt.src, &opt.out);
    // Produce an assembled output
    a.assemble();
}

/// Emulator for the KAP-16 processor.
#[derive(Clap)]
#[clap(author = crate_authors!())]
#[clap(version = crate_version!())]
struct Opt {
    /// Input source file
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    src: PathBuf,

    /// Output binary file
    #[clap(short, long, parse(from_os_str), default_value = "a.out", value_hint = ValueHint::FilePath)]
    out: PathBuf,

    /// Use verbose output (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
}
