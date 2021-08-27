use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read<P>(src: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let f = File::open(src)?;
    BufReader::new(f).lines().collect()
}
