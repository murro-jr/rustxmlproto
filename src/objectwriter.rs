use convert_case::{Case, Casing};
use std::fs::File;
use std::io::Write;

use crate::prototype::{Member, ObjectType, Prototype};

pub(crate) struct EnumFormatter;

impl EnumFormatter {
    pub fn format(members: Vec<Member>) -> String {
        let mut result = "".to_string();

        for member in members.iter() {
            if let Some(datatype) = &member.datatype {
                let member = match &datatype[..] {
                    "generic" => format!("\t{},\n", member.name.to_case(Case::UpperSnake)),
                    _ => format!(
                        "\t{}({}),\n",
                        member.name.to_case(Case::UpperSnake),
                        datatype
                    ),
                };

                result = result + &member;
            } else {
                panic!("Data type is unknown or undefined.");
            }
        }

        result
    }
}

pub(crate) struct StructFormatter;

impl StructFormatter {
    pub fn format(members: Vec<Member>) -> String {
        let mut result = "".to_string();

        for member in members.iter() {
            if let Some(datatype) = &member.datatype {
                let member = match &member.visibility {
                    Some(visibility) => {
                        format!("\tpub({}) {}: {},\n", visibility, member.name, datatype)
                    }
                    None => format!("\t{}: {},\n", member.name, datatype),
                };

                result = result + &member;
            } else {
                panic!("Data type is unknown or undefined.");
            }
        }

        result
    }
}

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
                let mut members = EnumFormatter::format(prototype.members);
                members = members + "}";
                file.write(members.as_bytes())?;
            }
            Ok(ObjectType::STRUCT) => {
                let mut members = StructFormatter::format(prototype.members);
                members = members + "}";
                file.write(members.as_bytes())?;
            }
            _ => {}
        };

        Ok(())
    }
}
