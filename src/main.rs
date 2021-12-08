mod formatter;
mod inheritwriter;
mod objectwriter;
mod preface;
mod prototype;
mod value;

extern crate proto_derive;

use serde_xml_rs::de::from_reader;
use std::fs::File;
use std::io::BufReader;

use crate::inheritwriter::InheritWriter;
use crate::objectwriter::ObjectWriter;
use crate::preface::Preface;
use crate::prototype::Prototype;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("File path of the xml not found or provided.");
    }

    let path = args[1].clone();
    let file = File::open(&path).expect(&format!(
        "The system cannot find the file specified: {}",
        path
    ));
    let reader = BufReader::new(file);

    let prototype: Result<Prototype, _> = from_reader(reader).map_err(|err| {
        println!("Error: {}", err);
    });

    if let Ok(prototype) = prototype {
        println!("{:?}", prototype);
        let prototype = Preface::new(prototype).prepare();
        let file = ObjectWriter::write(prototype.clone(), args[2].clone());

        if let Ok(file) = file {
            InheritWriter::write(file, prototype).unwrap();
        }
    }
}
