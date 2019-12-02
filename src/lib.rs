use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub fn parse_input<B, F>(mut f: F) -> Result<Vec<B>, io::Error>
where
    F: FnMut(String) -> B,
{
    let arg = env::args()
        .skip(1)
        .next()
        .ok_or(io::Error::from(io::ErrorKind::InvalidInput))?;

    let reader = BufReader::new(File::open(arg)?);

    reader
        .lines()
        .map(|line| line.map(|s| f(s)))
        .collect::<Result<Vec<B>, _>>()
}
