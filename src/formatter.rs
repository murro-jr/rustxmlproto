use convert_case::{Case, Casing};

use crate::prototype::{Function, Include, Member};
use crate::value::Visibility;

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
                        let visibility: Visibility = visibility.parse().unwrap();

                        match visibility {
                            Visibility::MODULE | Visibility::CRATE => {
                                format!(
                                    "\tpub({}) {}: {},\n",
                                    visibility.to_string(),
                                    member.name,
                                    datatype
                                )
                            }
                            Visibility::EXTERNAL => {
                                format!("\tpub {}: {},\n", member.name, datatype)
                            }
                            Visibility::PRIVATE => {
                                format!("\t{}: {},\n", member.name, datatype)
                            }
                        }
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

pub(crate) struct FunctionFormatter;

impl FunctionFormatter {
    pub fn format_header(function: &Function) -> String {
        let mut result = "".to_string();

        if let Some(visibility) = &function.visibility {
            let visibility: Visibility = visibility.parse().unwrap();

            match visibility {
                Visibility::MODULE | Visibility::CRATE => {
                    result = result + &format!("pub({}) ", visibility.to_string());
                }
                Visibility::EXTERNAL => {
                    result = result + "pub ";
                }
                Visibility::PRIVATE => {
                    println!("There is nothing to do here.");
                }
            };
        }

        if let Some(is_async) = &function.is_async {
            if *is_async {
                result = result + "async ";
            }
        }

        result = result + &format!("fn {} (", &function.name);

        if let Some(parameters) = &function.parameters {
            for param in parameters.0.iter() {
                let mutable = match &param.mutable {
                    Some(true) => "mut ",
                    _ => "",
                };

                if let Some(datatype) = &param.datatype {
                    result = result + &format!("{}: {}{}, ", param.name, mutable, datatype);
                } else {
                    if &param.name != "self" {
                        result = result + &format!("{}: {}T, ", param.name, mutable);
                    } else {
                        result = result + &format!("{}self, ", mutable);
                    }
                }
            }
        }

        result = result + ")";

        if let Some(datatype) = &function.datatype {
            result = result + &format!(" -> {}", datatype);
        }

        result
    }

    pub fn format_body(function: &Function) -> String {
        let mut result = "".to_string();

        if let Some(body) = &function.body {
            result = result + &body[..];
        }

        result
    }
}

pub(crate) struct TraitFormatter;

impl TraitFormatter {
    pub fn format(functions: Vec<Function>) -> String {
        let mut result = "".to_string();

        for function in functions.iter() {
            result = result + "\t" + &FunctionFormatter::format_header(function) + ";\n";
        }

        result
    }
}

pub(crate) struct ImplFormatter;

impl ImplFormatter {
    pub fn format(name: String, functions: Vec<Function>, inherits_to: Option<String>) -> String {
        let mut result = if let Some(inherits_to) = inherits_to {
            format!("impl {} for {} {{\n", name, inherits_to)
        } else {
            format!("impl {} {{\n", name)
        };

        for function in functions.iter() {
            result = result + "\t" + &FunctionFormatter::format_header(function) + " {\n";
            result = result + "\t\t" + &FunctionFormatter::format_body(function) + "\n\t}\n";
        }

        result = result + "}";
        result
    }
}

pub(crate) struct IncludeFormatter;

impl IncludeFormatter {
    pub fn format(includes: Vec<Include>) -> String {
        let mut result = "".to_string();

        for include in includes.iter() {
            let class = match &include.class[..] {
                "within" => "crate::",
                "extern" => "",
                _ => panic!("Include class declaration not supported."),
            };

            result = result + &format!("use {}{}", class, include.name);

            if let Some(scope) = &include.scope {
                if &scope[..] == "all" {
                    result = result + "::*;\n";
                }
            } else if let Some(objects) = &include.objects {
                result = result + "::{ " + &objects + " };\n";
            } else {
                result = result + ";\n";
            }
        }

        result
    }
}
