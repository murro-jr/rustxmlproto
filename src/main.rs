mod objectwriter;
mod prototype;

use serde_xml_rs::de::from_reader;
use std::fs::File;
use std::io::BufReader;

use crate::objectwriter::ObjectWriter;
use crate::prototype::Prototype;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("File path of the xml not found or provided.");
    }

    let file = File::open(&args[1][..]).expect("The system cannot find the file specified.");
    let reader = BufReader::new(file);

    let prototype: Prototype = from_reader(reader).unwrap();
    println!("{:?}", prototype);

    ObjectWriter::write(prototype).unwrap();
}
