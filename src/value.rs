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

#[derive(PartialEq)]
pub(crate) enum Visibility {
    PRIVATE,
    MODULE,
    CRATE,
    EXTERNAL,
}

impl std::str::FromStr for Visibility {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "mod" => Ok(Visibility::MODULE),
            "crate" => Ok(Visibility::CRATE),
            "external" | "global" => Ok(Visibility::EXTERNAL),
            _ => Ok(Visibility::PRIVATE),
        }
    }
}

impl std::string::ToString for Visibility {
    fn to_string(&self) -> String {
        match self {
            Self::MODULE => "mod".to_string(),
            Self::CRATE => "crate".to_string(),
            Self::EXTERNAL | Self::PRIVATE => "".to_string(),
        }
    }
}
