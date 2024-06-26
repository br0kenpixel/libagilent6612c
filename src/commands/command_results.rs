use crate::error::Error;
use std::{
    any::{type_name, Any},
    str::FromStr,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum CommandResult {
    Float(f32),
    String(String),
    Boolean(bool),
}

impl FromStr for CommandResult {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s.strip_suffix("\r\n").unwrap_or(s);

        if stripped == "1" {
            return Ok(Self::Boolean(true));
        }

        if stripped == "0" {
            return Ok(Self::Boolean(false));
        }

        if stripped.is_empty() {
            return Err(Error::EmptyResult);
        }

        if let Ok(float) = stripped.parse::<f32>() {
            return Ok(float.into());
        }

        Ok(stripped.into())
    }
}

impl From<f32> for CommandResult {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<&str> for CommandResult {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl TryFrom<CommandResult> for bool {
    type Error = crate::error::Error;

    fn try_from(value: CommandResult) -> Result<Self, Self::Error> {
        let CommandResult::Boolean(value) = value else {
            return Err(Self::Error::UnexpectedResultType {
                expected: "bool",
                got: value.type_name(),
            });
        };

        Ok(value)
    }
}

impl TryFrom<CommandResult> for f32 {
    type Error = crate::error::Error;

    fn try_from(value: CommandResult) -> Result<Self, Self::Error> {
        let CommandResult::Float(float) = value else {
            return Err(Self::Error::UnexpectedResultType {
                expected: "float",
                got: value.type_name(),
            });
        };

        Ok(float)
    }
}

impl TryFrom<CommandResult> for String {
    type Error = crate::error::Error;

    fn try_from(value: CommandResult) -> Result<Self, Self::Error> {
        let CommandResult::String(string) = value else {
            return Err(Self::Error::UnexpectedResultType {
                expected: "String",
                got: value.type_name(),
            });
        };

        Ok(string)
    }
}

impl CommandResult {
    pub(crate) fn type_name(&self) -> &'static str {
        match self {
            Self::Float(..) => type_name::<f32>(),
            Self::String(..) => type_name::<String>(),
            Self::Boolean(..) => type_name::<bool>(),
        }
    }

    pub(crate) fn into_any(self) -> Box<dyn Any> {
        match self {
            Self::Float(value) => Box::new(value),
            Self::String(value) => Box::new(value),
            Self::Boolean(value) => Box::new(value),
        }
    }
}
