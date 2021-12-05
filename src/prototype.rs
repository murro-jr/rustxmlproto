use serde;
use serde_derive::{Deserialize, Serialize};

pub(crate) enum ObjectType {
    STRUCT,
    TRAIT,
    ENUM,
}

impl std::str::FromStr for ObjectType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "struct" => Ok(ObjectType::STRUCT),
            "trait" => Ok(ObjectType::TRAIT),
            "enum" => Ok(ObjectType::ENUM),
            _ => Err(format!("'{}' is not a valid value for ObjectType", value)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Parameter {
    pub(crate) name: String,

    #[serde(skip_deserializing)]
    pub(crate) datatype: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct Parameters(pub(crate) Vec<Parameter>);

impl<'de> serde::Deserialize<'de> for Parameters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ParameterVisitor;

        impl<'d> serde::de::Visitor<'d> for ParameterVisitor {
            type Value = Vec<Parameter>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                f.write_str("Expecting a map of parameters")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'d>,
            {
                let mut parameters: Vec<Parameter> = Vec::new();

                while let Some((key, mut value)) = access.next_entry::<String, Parameter>()? {
                    if key != "generic" {
                        value.datatype = Some(key);
                    } else {
                        value.datatype = None;
                    }

                    parameters.push(value);
                }

                Ok(parameters)
            }
        }

        Ok(Parameters(deserializer.deserialize_map(ParameterVisitor)?))
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters(Vec::new())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Function {
    pub(crate) name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) visibility: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub(crate) parameters: Option<Parameters>,

    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub(crate) datatype: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) is_async: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Member {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) datatype: Option<String>,

    pub(crate) name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) visibility: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct Functions(pub(crate) Vec<Function>);

impl<'de> serde::Deserialize<'de> for Functions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FunctionVisitor;

        impl<'d> serde::de::Visitor<'d> for FunctionVisitor {
            type Value = Vec<Function>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                f.write_str("Expecting a map of functions")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'d>,
            {
                let mut functions: Vec<Function> = Vec::new();

                while let Some((key, mut value)) = access.next_entry::<String, Function>()? {
                    if key != "generic" {
                        value.datatype = Some(key);
                    } else {
                        value.datatype = None;
                    }

                    functions.push(value);
                }
                Ok(functions)
            }
        }

        Ok(Functions(deserializer.deserialize_map(FunctionVisitor)?))
    }
}

impl Default for Functions {
    fn default() -> Self {
        Functions(Vec::new())
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Members(pub(crate) Vec<Member>);

impl<'de> serde::Deserialize<'de> for Members {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MemberVisitor;

        impl<'d> serde::de::Visitor<'d> for MemberVisitor {
            type Value = Vec<Member>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                f.write_str("Expecting a map of members")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'d>,
            {
                let mut members: Vec<Member> = Vec::new();

                while let Some((key, mut value)) = access.next_entry::<String, Member>()? {
                    if key != "generic" {
                        value.datatype = Some(key);
                    } else {
                        value.datatype = None;
                    }

                    members.push(value);
                }
                Ok(members)
            }
        }

        Ok(Members(deserializer.deserialize_map(MemberVisitor)?))
    }
}

impl Default for Members {
    fn default() -> Self {
        Members(Vec::new())
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub(crate) struct Prototype {
    pub(crate) name: String,
    pub(crate) class: String,
    pub(crate) visibility: Option<String>,

    #[serde(default)]
    pub(crate) functions: Functions,

    #[serde(default)]
    pub(crate) members: Members,
}
