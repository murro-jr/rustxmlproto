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
                mutable: Some(false),
                container: None,
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
                    mutable: Some(true),
                    container: None,
                },
                Parameter {
                    name: member.name.clone(),
                    datatype: member.datatype.clone(),
                    mutable: Some(false),
                    container: None,
                },
            ];

            let body = format!("self.{} = {};\n\t\tself", member.name, member.name);

            functions.push(Function {
                name,
                datatype: Some("Self".to_string()),
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
                mutable: Some(false),
                container: None,
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

struct TraitPreface;

impl TraitPreface {
    fn prepend_self(functions: Vec<Function>) -> Vec<Function> {
        let mut new_functions: Vec<Function> = Vec::new();

        for function in functions.iter() {
            let mut parameters: Vec<Parameter> = vec![Parameter {
                name: "self".to_string(),
                datatype: None,
                mutable: Some(false),
                container: None,
            }];

            if let Some(params) = &function.parameters {
                parameters.extend(params.0.clone());
            }

            new_functions.push(Function {
                name: function.name.clone(),
                datatype: function.datatype.clone(),
                parameters: Some(Parameters(parameters)),
                body: function.body.clone(),
                visibility: function.visibility.clone(),
                is_async: function.is_async,
            });
        }

        new_functions
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
                let mut functions: Vec<Function> = Vec::new();
                let members = &self.prototype.members.0;
                let preface = StructPreface::new(members.to_vec());

                // Add the constructor method
                let constructor = preface.clone().constructor();
                functions.push(constructor);

                // Add the setter methods
                let setters = preface.clone().setters();
                functions.extend(setters);

                // Add the getter methods
                let getters = preface.getters();
                functions.extend(getters);

                functions.extend(self.prototype.functions.0);
                self.prototype.functions.0 = functions;
            }
            Ok(ObjectType::TRAIT) => {
                let functions = &self.prototype.functions.0;
                let functions = TraitPreface::prepend_self(functions.to_vec());
                self.prototype.functions.0 = functions;
            }
            _ => println!("We do nothing here yet."),
        };

        self.prototype
    }
}
