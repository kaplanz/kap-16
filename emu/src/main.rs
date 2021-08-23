use std::path::PathBuf;

use clap::{crate_authors, crate_version, Clap, ValueHint};
use emu::Emulator;
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

    // Instantiate an emulator
    let mut e = Emulator::new();
    // Load the ROM into memory
    e.load(&opt.rom).unwrap();
    // Run the emulator
    e.main();
}

/// Emulator for the KAP-16 processor.
#[derive(Clap)]
#[clap(author = crate_authors!())]
#[clap(version = crate_version!())]
struct Opt {
    /// Input ROM file
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    rom: PathBuf,

    /// Use verbose output (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
}
