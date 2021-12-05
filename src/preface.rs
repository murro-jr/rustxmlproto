use crate::prototype::{Function, Member, Parameter, Parameters, Prototype};
use crate::value::ObjectType;

struct StructPreface {
    members: Vec<Member>,
}

impl StructPreface {
    fn new(members: Vec<Member>) -> Self {
        Self { members }
    }

    fn constructor(self) -> Function {
        let mut parameters: Vec<Parameter> = Vec::new();
        let mut body = "Self { ".to_string();

        for member in self.members.iter() {
            body = body + &format!("{}, ", member.name);

            parameters.push(Parameter {
                name: member.name.clone(),
                datatype: member.datatype.clone(),
            });
        }

        body = body + "}";

        Function {
            name: "new".to_string(),
            datatype: Some("Self".to_string()),
            parameters: Some(Parameters(parameters)),
            body: Some(body),
            visibility: Some("crate".to_string()),
            is_async: Some(false),
        }
    }

    fn setters(self) -> Result<Vec<Function>, String> {
        //TODO: add the setter methods and collect into vector of function
        Ok(Vec::new())
    }

    fn getters(self) -> Result<Vec<Function>, String> {
        //TODO: add the getter methods and collect into vector of function
        Ok(Vec::new())
    }
}

pub(crate) struct Preface {
    prototype: Prototype,
}

impl Preface {
    pub(crate) fn new(prototype: Prototype) -> Self {
        Self { prototype }
    }

    pub(crate) fn prepare(mut self) -> Prototype {
        match self.prototype.class.parse::<ObjectType>() {
            Ok(ObjectType::STRUCT) => {
                // Add the constructor method
                let members = &self.prototype.members.0;
                let constructor = StructPreface::new(members.to_vec()).constructor();
                self.prototype.functions.0.push(constructor)
            }
            _ => println!("We do nothing here yet."),
        };

        self.prototype
    }
}
