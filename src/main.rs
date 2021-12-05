mod formatter;
mod objectwriter;
mod preface;
mod prototype;
mod value;

extern crate proto_derive;

use serde_xml_rs::de::from_reader;
use std::fs::File;
use std::io::BufReader;

use crate::objectwriter::ObjectWriter;
use crate::preface::Preface;
use crate::prototype::Prototype;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("File path of the xml not found or provided.");
    }

    let file = File::open(&args[1][..]).expect("The system cannot find the file specified.");
    let reader = BufReader::new(file);

    let prototype: Result<Prototype, _> = from_reader(reader).map_err(|err| {
        println!("Error: {}", err);
    });

    if let Ok(prototype) = prototype {
        println!("{:?}", prototype);
        let prototype = Preface::new(prototype).prepare();
        ObjectWriter::write(prototype).unwrap();
    }
}
