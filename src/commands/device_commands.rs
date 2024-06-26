use crate::error::Result;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Command {
    GetDeviceModelName,
    GetFirmwareVersion,
    SetOutput(bool),
}

impl Command {
    pub fn serialize(self) -> Result<Box<str>> {
        let mut buffer = String::with_capacity(4);

        match self {
            Self::GetDeviceModelName => buffer.push_str("*id?"),
            Self::GetFirmwareVersion => unimplemented!(),
            Self::SetOutput(value) => {
                buffer.push_str("output ");

                if value {
                    buffer.push_str("on");
                } else {
                    buffer.push_str("off");
                }
            }
        };

        buffer.push_str("\r\n");

        Ok(buffer.into_boxed_str())
    }
}
