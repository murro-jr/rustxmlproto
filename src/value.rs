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

#[derive(PartialEq)]
pub(crate) enum CommonMacro {
    DERIVE,
    SERDE,
    CUSTOM,
}

impl std::str::FromStr for CommonMacro {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "derive" => Ok(CommonMacro::DERIVE),
            "serde" => Ok(CommonMacro::SERDE),
            "custom" => Ok(CommonMacro::CUSTOM),
            _ => panic!("'{}' is not supported as tag for macro", value),
        }
    }
}

impl std::string::ToString for CommonMacro {
    fn to_string(&self) -> String {
        match self {
            Self::DERIVE => "derive".to_string(),
            Self::SERDE => "serde".to_string(),
            Self::CUSTOM => "".to_string(),
        }
    }
}
