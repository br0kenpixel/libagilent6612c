use crate::{
    commands::{Command, CommandResult},
    error::Result,
    params::ConnectionParameters,
};
use serialport::SerialPort;
use std::{str::FromStr, time::Duration};

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
        let result = self.send_command(Command::GetDeviceModelName)?;
        let result: String = result.try_into()?;

        Ok(result)
    }

    pub fn send_command(&mut self, cmd: Command) -> Result<CommandResult> {
        let raw = cmd.serialize()?;
        self.port.write_all(raw.as_bytes())?;

        let mut rcv_buf = [0; RCV_BUFSIZE];
        let read = self.port.read(&mut rcv_buf)?;

        let received_raw = &rcv_buf[..read];
        let received_string = std::str::from_utf8(received_raw)?;
        let result = CommandResult::from_str(received_string)?;

        Ok(result)
    }
}
