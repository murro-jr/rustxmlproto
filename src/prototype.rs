use serde;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Function {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    return_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    is_async: Option<bool>,
}

fn deserialize_functions<'de, T>(t: T) -> Result<Vec<Function>, T::Error>
where
    T: serde::Deserializer<'de>,
{
    let functions: HashMap<String, Function> = serde::Deserialize::deserialize(t)?;
    let functions: Vec<Function> = functions
        .into_iter()
        .map(|(name, item)| Function {
            name: Some(name),
            return_type: item.return_type,
            is_async: item.is_async,
        })
        .collect();
    Ok(functions)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Prototype {
    pub(crate) name: String,
    pub(crate) class: String,
    pub(crate) visibility: Option<String>,

    #[serde(deserialize_with = "deserialize_functions")]
    functions: Vec<Function>,
}
