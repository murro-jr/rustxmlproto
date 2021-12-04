use crate::prototype::{Function, Member};
use convert_case::{Case, Casing};

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

pub(crate) struct FunctionFormatter;

impl FunctionFormatter {
    pub fn format(function: &Function) -> String {
        let mut result = "".to_string();

        if let Some(visibility) = &function.visibility {
            result = result + &format!("pub({}) ", visibility);
        }

        if let Some(is_async) = &function.is_async {
            if *is_async {
                result = result + "async ";
            }
        }

        result = result + &format!("fn {} (self, ", &function.name);

        if let Some(parameters) = &function.parameters {
            for param in parameters.iter() {
                result = result + &format!("{}: {}, ", param.name, param.datatype);
            }
        }

        result = result + ")";

        if let Some(return_type) = &function.return_type {
            result = result + &format!(" -> {}", return_type);
        }

        result
    }
}

pub(crate) struct TraitFormatter;

impl TraitFormatter {
    pub fn format(functions: Vec<Function>) -> String {
        let mut result = "".to_string();

        for function in functions.iter() {
            result = result + "\t" + &FunctionFormatter::format(function) + ";\n";
        }

        result
    }
}
