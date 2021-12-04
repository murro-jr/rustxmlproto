use serde;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub(crate) datatype: String,
}

fn deserialize_parameters<'de, T>(t: T) -> Result<Option<Vec<Parameter>>, T::Error>
where
    T: serde::Deserializer<'de>,
{
    let parameters: HashMap<String, Parameter> = serde::Deserialize::deserialize(t)?;
    let parameters: Vec<Parameter> = parameters
        .into_iter()
        .map(|(datatype, item)| Parameter {
            name: item.name,
            datatype: datatype,
        })
        .collect();
    Ok(Some(parameters))
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Function {
    #[serde(skip_deserializing)]
    pub(crate) name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) visibility: Option<String>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_parameters"
    )]
    pub(crate) parameters: Option<Vec<Parameter>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) return_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) is_async: Option<bool>,
}

fn deserialize_functions<'de, T>(t: T) -> Result<Vec<Function>, T::Error>
where
    T: serde::Deserializer<'de>,
{
    let functions: HashMap<String, Function> = serde::Deserialize::deserialize(t)?;
    let functions: Vec<Function> = functions
        .into_iter()
        .map(|(name, item)| Function {
            name: name,
            visibility: item.visibility,
            parameters: item.parameters,
            return_type: item.return_type,
            is_async: item.is_async,
        })
        .collect();
    Ok(functions)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Member {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) datatype: Option<String>,

    pub(crate) name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) visibility: Option<String>,
}

fn deserialize_members<'de, T>(t: T) -> Result<Vec<Member>, T::Error>
where
    T: serde::Deserializer<'de>,
{
    let members: HashMap<String, Member> = serde::Deserialize::deserialize(t)?;
    let members: Vec<Member> = members
        .into_iter()
        .map(|(datatype, item)| Member {
            datatype: Some(datatype),
            visibility: item.visibility,
            name: item.name,
        })
        .collect();
    Ok(members)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub(crate) struct Prototype {
    pub(crate) name: String,
    pub(crate) class: String,
    pub(crate) visibility: Option<String>,

    #[serde(deserialize_with = "deserialize_functions")]
    pub(crate) functions: Vec<Function>,

    #[serde(deserialize_with = "deserialize_members")]
    pub(crate) members: Vec<Member>,
}
