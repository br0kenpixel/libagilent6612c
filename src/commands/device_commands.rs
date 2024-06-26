use crate::error::Result;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Command {
    GetDeviceModelName,
    GetFirmwareVersion,
    SetOutput(bool),
    GetOutput,
}

impl Command {
    pub fn serialize(self) -> Result<Box<str>> {
        let mut buffer = String::with_capacity(4);

        match self {
            Self::GetDeviceModelName => buffer.push_str("*idn?"),
            Self::GetFirmwareVersion => unimplemented!(),
            Self::SetOutput(value) => {
                buffer.push_str("output ");

                if value {
                    buffer.push_str("on");
                } else {
                    buffer.push_str("off");
                }
            }
            Self::GetOutput => buffer.push_str("output?"),
        };

        buffer.push_str("\r\n");

        Ok(buffer.into_boxed_str())
    }
}
