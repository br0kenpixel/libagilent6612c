use crate::{
    commands::{Command, CommandResult},
    error::Result,
    params::ConnectionParameters,
};
use serialport::SerialPort;
use std::{any::type_name, io::ErrorKind, str::FromStr, time::Duration};

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(3);
pub const RCV_BUFSIZE: usize = 128;

pub struct Agilent6612c {
    port: Box<dyn SerialPort>,
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

        Ok(Self { port: connection })
    }

    pub fn hwinfo(&mut self) -> Result<String> {
        self.send_command_and_expect_result(Command::GetDeviceModelName, false)
    }

    pub fn output(&mut self) -> Result<bool> {
        self.send_command_and_expect_result(Command::GetOutput, false)
    }

    pub fn measure_voltage(&mut self) -> Result<f32> {
        self.send_command_and_expect_result(Command::MeasureVoltage, false)
    }

    pub fn measure_current(&mut self) -> Result<f32> {
        self.send_command_and_expect_result(Command::MeasureCurrent, false)
    }

    pub fn output_voltage(&mut self) -> Result<f32> {
        self.send_command_and_expect_result(Command::GetOutputVoltage, false)
    }

    pub fn output_current(&mut self) -> Result<f32> {
        self.send_command_and_expect_result(Command::GetOutputCurrent, false)
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

    fn send_command_and_expect_result<T: Sized + 'static>(
        &mut self,
        cmd: Command,
        allow_timeout: bool,
    ) -> Result<T> {
        let result = self.send_command_with_result(cmd, allow_timeout)?;
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

    pub fn send_command_with_result(
        &mut self,
        cmd: Command,
        allow_timeout: bool,
    ) -> Result<CommandResult> {
        self.send_command(cmd)?;

        let mut rcv_buf = [0; RCV_BUFSIZE];
        let read = match self.port.read(&mut rcv_buf) {
            Ok(amount) => amount,
            Err(why) if allow_timeout && why.kind() == ErrorKind::TimedOut => {
                return Ok(CommandResult::Nothing)
            }
            Err(why) => return Err(why.into()),
        };

        let received_raw = &rcv_buf[..read];
        let received_string = std::str::from_utf8(received_raw)?;
        let result = CommandResult::from_str(received_string)?;

        Ok(result)
    }

    pub fn send_command(&mut self, cmd: Command) -> Result<()> {
        let raw = cmd.serialize()?;
        self.port.write_all(raw.as_bytes())?;

        Ok(())
    }
}
