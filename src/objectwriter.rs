use convert_case::{Case, Casing};
use std::fs::File;
use std::io::Write;

use crate::prototype::Prototype;

pub(crate) struct ObjectWriter;

impl ObjectWriter {
    pub fn write(prototype: Prototype) -> std::io::Result<()> {
        let path = format!("./{}.rs", prototype.name.to_case(Case::Snake));
        let mut file = File::create(path)?;

        match prototype.visibility {
            Some(visibility) => {
                //Create an object based on visibility
                let object = format!("pub({}) {} {};", visibility, prototype.class, prototype.name);
                file.write(object.as_bytes())?;
            }
            None => {
                //Defaults to private object
                let object = format!("{} {};", prototype.class, prototype.name);
                file.write(object.as_bytes())?;
            }
        };

        Ok(())
    }
}
