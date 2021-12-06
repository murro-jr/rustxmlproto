use convert_case::{Case, Casing};

use crate::prototype::{Function, Member, Parameter, Parameters, Prototype};
use crate::value::ObjectType;

#[derive(Clone)]
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

    fn setters(self) -> Vec<Function> {
        let mut functions: Vec<Function> = Vec::new();

        for member in self.members.iter() {
            let name = format!("set_{}", member.name.to_case(Case::Snake));
            let parameters: Vec<Parameter> = vec![
                Parameter {
                    name: "self".to_string(),
                    datatype: None,
                },
                Parameter {
                    name: member.name.clone(),
                    datatype: member.datatype.clone(),
                },
            ];

            let body = format!("self.{} = {};", member.name, member.name);

            functions.push(Function {
                name,
                datatype: None,
                parameters: Some(Parameters(parameters)),
                body: Some(body),
                visibility: Some("crate".to_string()),
                is_async: Some(false),
            })
        }

        functions
    }

    fn getters(self) -> Vec<Function> {
        let mut functions: Vec<Function> = Vec::new();

        for member in self.members.iter() {
            let name = format!("get_{}", member.name.to_case(Case::Snake));
            let body = format!("self.{}", member.name);
            let parameters: Vec<Parameter> = vec![Parameter {
                name: "self".to_string(),
                datatype: None,
            }];

            functions.push(Function {
                name,
                datatype: member.datatype.clone(),
                parameters: Some(Parameters(parameters)),
                body: Some(body),
                visibility: Some("crate".to_string()),
                is_async: Some(false),
            })
        }

        functions
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
                let preface = StructPreface::new(members.to_vec());

                let constructor = preface.clone().constructor();
                self.prototype.functions.0.push(constructor);

                let setters = preface.clone().setters();
                self.prototype.functions.0.extend(setters);

                let getters = preface.getters();
                self.prototype.functions.0.extend(getters);
            }
            _ => println!("We do nothing here yet."),
        };

        self.prototype
    }
}