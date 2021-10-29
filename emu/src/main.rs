use std::path::PathBuf;
use std::process;

use clap::{crate_authors, crate_version, Parser, ValueHint};
use emu::Emulator;
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

    // Instantiate an emulator
    let mut e = Emulator::new();
    // Load the ROM into memory
    e.load(&opts.rom).unwrap_or_else(|err| {
        error!("`{}`: {}", &opts.rom.display(), err);
        process::exit(1)
    });
    // Run the emulator
    e.main();
}

/// Emulator for the KAP-16 processor.
#[derive(Parser, Debug)]
#[clap(author = crate_authors!())]
#[clap(version = crate_version!())]
struct Opts {
    /// Input ROM file
    #[clap(parse(from_os_str))]
    #[clap(value_hint = ValueHint::FilePath)]
    rom: PathBuf,

    /// Use verbose output (-v, -vv, -vvv, etc.)
    #[clap(short)]
    #[clap(long)]
    #[clap(parse(from_occurrences))]
    verbose: u8,
}
