use serde_xml_rs::de::from_reader;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

use crate::formatter::ImplFormatter;
use crate::prototype::Prototype;

pub(crate) struct InheritWriter;

impl InheritWriter {
    pub fn write(mut file: File, prototype: Prototype) -> std::io::Result<()> {
        for inherit in prototype.inherits.0 {
            let path = format!("{}.xml", inherit.name);
            let inherited_file = File::open(&path).expect(&format!(
                "The system cannot find the file specified: {}",
                path
            ));
            let reader = BufReader::new(inherited_file);

            let inherited_proto: Result<Prototype, _> = from_reader(reader).map_err(|err| {
                println!("Error: {}", err);
            });

            if let Ok(inherited_proto) = inherited_proto {
                println!("{:?}", inherited_proto);
                let mut output = "\n\n".to_string();
                output = output
                    + &ImplFormatter::format(
                        inherit.name,
                        inherited_proto.functions.0,
                        Some(prototype.name.clone()),
                    );
                file.write(output.as_bytes())?;
            }
        }
        Ok(())
    }
}
