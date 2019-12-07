use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_input() -> Result<Vec<String>, io::Error> {
    env::args()
        .skip(1)
        .take(1)
        .map(|arg| BufReader::new(File::open(arg)?).lines().collect())
        .collect()
}
