use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::{thread, time};

pub mod intcode_computer;

pub fn read_input() -> Vec<String> {
    let arg = env::args().skip(1).next().unwrap();
    BufReader::new(File::open(arg).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

pub fn delay(millis: u64) {
    let millis = time::Duration::from_millis(millis);
    thread::sleep(millis);
}
