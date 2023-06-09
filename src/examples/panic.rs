#![allow(unused)] // ignore unused variable warnings

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let path = "files/lines.txt";

    let output = File::create(path);

    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error);
        }
    };

    write!(output, "Just some\nRandom words").expect("Failed to write to file");

    let input = File::open(path).unwrap();

    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("files/random text.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("files/rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", error),
            },
            _other_error => panic!("Problem opening file: {:?}", error),
        },
    };
}
