use convert_case::{Case, Casing};

use crate::prototype::{Function, Include, Member, ProcMacro};
use crate::value::{CommonMacro, Visibility};

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
            let procs = ProcFormatter::format(member.procs.0.to_vec(), true);
            result = result + &procs;
            if let Some(datatype) = &member.datatype {
                let contained = if let Some(container) = &member.container {
                    format!("{}<{}>", container, datatype)
                } else {
                    datatype.clone()
                };

                let member = match &member.visibility {
                    Some(visibility) => {
                        let visibility: Visibility = visibility.parse().unwrap();

                        match visibility {
                            Visibility::MODULE | Visibility::CRATE => {
                                format!(
                                    "\tpub({}) {}: {},\n",
                                    visibility.to_string(),
                                    member.name,
                                    contained
                                )
                            }
                            Visibility::EXTERNAL => {
                                format!("\tpub {}: {},\n", member.name, contained)
                            }
                            Visibility::PRIVATE => {
                                format!("\t{}: {},\n", member.name, contained)
                            }
                        }
                    }
                    None => format!("\t{}: {},\n", member.name, contained),
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

                if &param.name != "self" {
                    let generic = "T".to_string();
                    let datatype = param.datatype.as_ref().unwrap_or(&generic);
                    let contained = param
                        .container
                        .as_ref()
                        .map(|data| format!("{}<{}>", data, datatype))
                        .unwrap_or(datatype.to_string());

                    result = result + &format!("{}: {}{}, ", param.name, mutable, contained);
                } else {
                    result = result + &format!("{}self, ", mutable);
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

pub(crate) struct ProcFormatter;

impl ProcFormatter {
    pub fn format(procs: Vec<ProcMacro>, indented: bool) -> String {
        let mut result = "".to_string();

        for proc in procs {
            let class = proc.class.parse::<CommonMacro>().unwrap();
            let set = proc
                .set
                .map(|data| {
                    if let Some(value) = proc.value {
                        format!("({} = \"{}\")", data, value)
                    } else {
                        format!("({})", data)
                    }
                })
                .unwrap_or("".to_string());

            let output = match class {
                CommonMacro::DERIVE | CommonMacro::SERDE => {
                    format!("#[{}{}]\n", class.to_string(), set)
                }
                CommonMacro::CUSTOM => {
                    if let Some(name) = proc.name {
                        format!("#[{}{}]\n", name, set)
                    } else {
                        "".to_string()
                    }
                }
            };

            if indented {
                result = result + "\t" + &output;
            } else {
                result = result + &output;
            }
        }

        result
    }
}
