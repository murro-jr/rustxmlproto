use convert_case::{Case, Casing};
use std::fs::File;
use std::io::Write;

use crate::formatter::{EnumFormatter, ImplFormatter, StructFormatter, TraitFormatter};
use crate::prototype::{ObjectType, Prototype};

pub(crate) struct ObjectWriter;

impl ObjectWriter {
    pub fn write(prototype: Prototype) -> std::io::Result<()> {
        let path = format!("./{}.rs", prototype.name.to_case(Case::Snake));
        let mut file = File::create(path)?;

        match prototype.visibility {
            Some(visibility) => {
                //Create an object based on visibility
                let object = format!(
                    "pub({}) {} {} {{\n",
                    visibility, prototype.class, prototype.name
                );
                file.write(object.as_bytes())?;
            }
            None => {
                //Defaults to private object
                let object = format!("{} {} {{\n", prototype.class, prototype.name);
                file.write(object.as_bytes())?;
            }
        };

        let object_type: Result<ObjectType, String> = prototype.class.parse();

        match object_type {
            Ok(ObjectType::ENUM) => {
                let mut members = EnumFormatter::format(prototype.members.0);
                members = members + "}";
                file.write(members.as_bytes())?;
            }
            Ok(ObjectType::STRUCT) => {
                let mut output = StructFormatter::format(prototype.members.0);
                output = output + "}";
                output =
                    output + "\n\n" + &ImplFormatter::format(prototype.name, prototype.functions.0);
                file.write(output.as_bytes())?;
            }
            Ok(ObjectType::TRAIT) => {
                let mut functions = TraitFormatter::format(prototype.functions.0);
                functions = functions + "}";
                file.write(functions.as_bytes())?;
            }
            Err(err) => panic!("Error parsing ObjectType: {}", err),
        };

        Ok(())
    }
}
