use std::{thread::sleep, time::Duration};

use agilent6612c::{device::Agilent6612c, params::ConnectionParameters};

fn main() {
    let params = ConnectionParameters::default();
    let mut device = Agilent6612c::new("/dev/cu.usbserial-14130", params, None).unwrap();

    println!("Connected");

    let out = device.output().unwrap();
    println!("The output is currently: {out}");

    device.set_output(!out).unwrap();
    sleep(Duration::from_secs(3));
    device.set_output(out).unwrap();
}
