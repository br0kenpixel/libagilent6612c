use derive_builder::Builder;
use serialport::{FlowControl, Parity, StopBits};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Baudrate {
    Baud300,
    Baud600,
    Baud1200,
    Baud2400,
    Baud4800,
    Baud9600,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Builder)]
#[builder(setter(into))]
pub struct ConnectionParameters {
    pub(crate) baudrate: Baudrate,
    pub(crate) parity: Parity,
    pub(crate) flow: FlowControl,
    pub(crate) stop_bits: StopBits,
}

impl Default for ConnectionParameters {
    fn default() -> Self {
        Self {
            baudrate: Baudrate::Baud9600,
            parity: Parity::None,
            flow: FlowControl::None,
            stop_bits: StopBits::One,
        }
    }
}

impl From<Baudrate> for u32 {
    fn from(val: Baudrate) -> Self {
        match val {
            Baudrate::Baud300 => 300,
            Baudrate::Baud600 => 600,
            Baudrate::Baud1200 => 1200,
            Baudrate::Baud2400 => 2400,
            Baudrate::Baud4800 => 4800,
            Baudrate::Baud9600 => 9600,
        }
    }
}
