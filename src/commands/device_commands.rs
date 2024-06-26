use crate::error::Result;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Command {
    GetDeviceModelName,
    GetFirmwareVersion,

    SetOutput(bool),
    GetOutput,

    MeasureVoltage,
    MeasureCurrent,

    GetOutputVoltage,
    GetOutputCurrent,
    SetOutputVoltage(f32, usize),
    SetOutputCurrent(f32, usize),
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
            Self::MeasureVoltage => buffer.push_str("meas:volt?"),
            Self::MeasureCurrent => buffer.push_str("meas:curr?"),
            Self::GetOutputVoltage => buffer.push_str("voltage?"),
            Self::GetOutputCurrent => buffer.push_str("current?"),
            Self::SetOutputVoltage(v, precision) => {
                buffer.push_str(&format!("voltage {v:.*}", precision));
            }
            Self::SetOutputCurrent(i, precision) => {
                buffer.push_str(&format!("current {i:.*}", precision));
            }
        };

        buffer.push_str("\r\n");

        Ok(buffer.into_boxed_str())
    }
}
