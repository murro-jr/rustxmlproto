use serde;
use serde_derive::{Deserialize, Serialize};

use proto_derive::ProtoDeserializer;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(crate) struct Parameter {
    pub(crate) name: String,

    #[serde(skip_deserializing)]
    pub(crate) datatype: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mutable: Option<bool>,
}

#[derive(Debug, Serialize, ProtoDeserializer, Clone)]
pub(crate) struct Parameters(pub(crate) Vec<Parameter>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Function {
    pub(crate) name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) visibility: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub(crate) parameters: Option<Parameters>,

    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub(crate) datatype: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) body: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) is_async: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(crate) struct Member {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) datatype: Option<String>,

    pub(crate) name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) visibility: Option<String>,
}

#[derive(Debug, Serialize, ProtoDeserializer)]
pub(crate) struct Functions(pub(crate) Vec<Function>);

#[derive(Debug, Serialize, ProtoDeserializer)]
pub(crate) struct Members(pub(crate) Vec<Member>);

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
