use crate::{
    commands::{Command, CommandResult},
    error::Result,
    params::ConnectionParameters,
};
use serialport::SerialPort;
use std::{
    any::type_name,
    io::{BufRead, BufReader, BufWriter, Write},
    str::FromStr,
    time::Duration,
};

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(3);
pub const RCV_BUFSIZE: usize = 128;

pub struct Agilent6612c {
    reader: BufReader<Box<dyn SerialPort>>,
    writer: BufWriter<Box<dyn SerialPort>>,
}

impl Agilent6612c {
    pub fn new<S: AsRef<str>>(
        port: S,
        params: ConnectionParameters,
        timeout: Option<Duration>,
    ) -> Result<Self> {
        let port_name = port.as_ref();

        let connection = serialport::new(port_name, params.baudrate.into())
            .parity(params.parity)
            .flow_control(params.flow)
            .stop_bits(params.stop_bits)
            .timeout(timeout.unwrap_or(DEFAULT_TIMEOUT))
            .open()?;
        let copy = connection.try_clone()?;

        Ok(Self {
            reader: BufReader::new(connection),
            writer: BufWriter::new(copy),
        })
    }

    pub fn hwinfo(&mut self) -> Result<String> {
        self.send_command_and_expect_result(Command::GetDeviceModelName)
    }

    pub fn maximum_voltage(&mut self) -> Result<f32> {
        self.send_command_and_expect_result(Command::GetMaximumSupportedVoltage)
    }

    pub fn maximum_current(&mut self) -> Result<f32> {
        self.send_command_and_expect_result(Command::GetMaximumSupportedCurrent)
    }

    pub fn output(&mut self) -> Result<bool> {
        self.send_command_and_expect_result(Command::GetOutput)
    }

    pub fn measure_voltage(&mut self) -> Result<f32> {
        self.send_command_and_expect_result(Command::MeasureVoltage)
    }

    pub fn measure_current(&mut self) -> Result<f32> {
        self.send_command_and_expect_result(Command::MeasureCurrent)
    }

    pub fn output_voltage(&mut self) -> Result<f32> {
        self.send_command_and_expect_result(Command::GetOutputVoltage)
    }

    pub fn output_current(&mut self) -> Result<f32> {
        self.send_command_and_expect_result(Command::GetOutputCurrent)
    }

    pub fn set_output(&mut self, enabled: bool) -> Result<()> {
        self.send_command(Command::SetOutput(enabled))
    }

    pub fn set_output_voltage(&mut self, value: f32) -> Result<()> {
        self.send_command(Command::SetOutputVoltage(value, 4))
    }

    pub fn set_output_current(&mut self, value: f32) -> Result<()> {
        self.send_command(Command::SetOutputCurrent(value, 4))
    }

    fn send_command_and_expect_result<T: Sized + 'static>(&mut self, cmd: Command) -> Result<T> {
        let result = self.send_command_with_result(cmd)?;
        let got = result.type_name();

        let any_result = result.into_any();

        match any_result.downcast::<T>() {
            Ok(value) => Ok(*value),
            Err(_) => Err(crate::error::Error::UnexpectedResultType {
                expected: type_name::<T>(),
                got,
            }),
        }
    }

    pub fn send_command_with_result(&mut self, cmd: Command) -> Result<CommandResult> {
        self.send_command(cmd)?;

        let mut buffer = String::with_capacity(12);
        self.reader.read_line(&mut buffer)?;

        let result = CommandResult::from_str(&buffer)?;

        Ok(result)
    }

    pub fn send_command(&mut self, cmd: Command) -> Result<()> {
        let raw = cmd.serialize()?;
        self.writer.write_all(raw.as_bytes())?;
        self.writer.flush()?;

        Ok(())
    }
}
