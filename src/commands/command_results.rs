use std::{any::type_name, str::FromStr};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum CommandResult {
    Nothing,
    Float(f32),
    String(String),
}

impl FromStr for CommandResult {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s.strip_suffix("\r\n").unwrap_or(s);

        if stripped.is_empty() {
            return Ok(Self::Nothing);
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
    fn type_name(&self) -> &'static str {
        match self {
            Self::Nothing => "NULL",
            Self::Float(..) => type_name::<f32>(),
            Self::String(..) => type_name::<String>(),
        }
    }
}
