use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn read_input() -> Result<Vec<String>, io::Error> {
    let arg = env::args()
        .skip(1)
        .next()
        .ok_or(io::Error::from(io::ErrorKind::InvalidInput))?;
    BufReader::new(File::open(arg)?).lines().collect()
}
